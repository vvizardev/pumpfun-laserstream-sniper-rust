use crate::{
    states::{BondingCurve, InitializeConfiguration},
    SUPPLY, CreateMintAccountArgs,
};
use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, program::invoke, system_instruction},
    system_program::{self},
};
use anchor_spl::{
    associated_token::{
        spl_associated_token_account::instruction::{self},
        AssociatedToken,
    },
    token_2022::{
        self,
        spl_token_2022::{
            self,
            extension::{
                transfer_fee::TransferFeeConfig, BaseStateWithExtensions, ExtensionType,
                StateWithExtensions,
            },
            state::Mint as MintState,
        },
        Token2022,
    },
    token_interface::{
        self, initialize_mint2, mint_to, spl_pod::optional_keys::OptionalNonZeroPubkey,
        token_metadata_initialize, transfer_fee_initialize, InitializeMint2, MintTo, SetAuthority,
        TokenInterface, TokenMetadataInitialize, TransferFeeInitialize,
    },
};

#[derive(Accounts)]
#[instruction(args: CreateMintAccountArgs)]
pub struct Create<'info> {
    #[account(
        seeds = [ b"global_config"],
        bump
    )]
    pub global_configuration: Box<Account<'info, InitializeConfiguration>>,

    #[account(
        init,
        payer = payer,
        seeds =[ &mint_addr.key().to_bytes() , BondingCurve::POOL_SEED_PREFIX ],
        space = BondingCurve::SIZE,
        bump
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    /// CHECK:
    #[account(mut)]
    pub mint_addr: Signer<'info>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ &mint_addr.key().to_bytes() , b"sol_pool".as_ref() ],
        bump
    )]
    pub sol_pool: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub token_pool: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub fee_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    fn initialize_token_metadata(
        &self,
        name: String,
        symbol: String,
        uri: String,
    ) -> ProgramResult {
        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint_addr.to_account_info(),
            metadata: self.mint_addr.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.payer.to_account_info(),
            update_authority: self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
        Ok(())
    }

    fn set_freeze_authority(&self) -> ProgramResult {
        let cpi_accounts_frz = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.mint_addr.to_account_info(),
        };
        let cpi_ctx_frz = CpiContext::new(self.token_program.to_account_info(), cpi_accounts_frz);

        token_interface::set_authority(
            cpi_ctx_frz,
            token_interface::spl_token_2022::instruction::AuthorityType::FreezeAccount,
            None,
        )?;

        Ok(())
    }

    fn set_mint_authority(&self) -> ProgramResult {
        let cpi_accounts_frz = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.mint_addr.to_account_info(),
        };
        let cpi_ctx_frz = CpiContext::new(self.token_program.to_account_info(), cpi_accounts_frz);

        token_interface::set_authority(
            cpi_ctx_frz,
            token_interface::spl_token_2022::instruction::AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }

    fn mint_tokens(&self, supply_arg: u64) -> ProgramResult {
        mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    authority: self.payer.to_account_info(),
                    to: self.token_pool.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
            ),
            supply_arg,
        )?;
        Ok(())
    }

    fn transfer_fee_to_fee_account(&self) -> ProgramResult {
        let transfer_instruction = system_instruction::transfer(
            &self.payer.to_account_info().key(),
            &self.fee_account.to_account_info().key(),
            self.global_configuration.create_pool_fee_lamports,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                self.payer.to_account_info(),
                self.fee_account.clone(),
                self.system_program.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn check_mint_data(&self) -> Result<()> {
        let mint = &self.mint_addr.to_account_info();
        let mint_data = mint.data.borrow();
        let mint_with_extension = StateWithExtensions::<MintState>::unpack(&mint_data)?;
        let extension_data = mint_with_extension.get_extension::<TransferFeeConfig>()?;

        assert_eq!(
            extension_data.transfer_fee_config_authority,
            OptionalNonZeroPubkey::try_from(Some(self.payer.key()))?
        );

        assert_eq!(
            extension_data.withdraw_withheld_authority,
            OptionalNonZeroPubkey::try_from(Some(self.payer.key()))?
        );

        msg!("{:?}", extension_data);
        Ok(())
    }

    pub fn process(&mut self, args: CreateMintAccountArgs) -> Result<()> {
        let space = ExtensionType::try_calculate_account_len::<MintState>(&[
            ExtensionType::MetadataPointer,
            ExtensionType::TransferFeeConfig,
        ])
        .unwrap();

        // This is the space required for the metadata account.
        // We put the meta data into the mint account at the end so we
        // don't need to create and additional account.
        let meta_data_space = 250;

        let lamports_required = (Rent::get()?).minimum_balance(space + meta_data_space);
        msg!(
            "Create Mint and metadata account size and cost: {} lamports: {}",
            space as u64,
            lamports_required
        );

        system_program::create_account(
            CpiContext::new(
                self.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: self.payer.to_account_info(),
                    to: self.mint_addr.to_account_info(),
                },
            ),
            lamports_required,
            space as u64,
            &self.token_program.key(),
        )?;

        // Assign the mint to the token program
        system_program::assign(
            CpiContext::new(
                self.token_program.to_account_info(),
                system_program::Assign {
                    account_to_assign: self.mint_addr.to_account_info(),
                },
            ),
            &token_2022::ID,
        )?;

        // Initialize the metadata pointer (Need to do this before initializing the mint)
        let init_meta_data_pointer_ix =
            spl_token_2022::extension::metadata_pointer::instruction::initialize(
                &Token2022::id(),
                &self.mint_addr.key(),
                Some(self.payer.key()),
                Some(self.mint_addr.key()),
            )
            .unwrap();

        invoke(
            &init_meta_data_pointer_ix,
            &[
                self.mint_addr.to_account_info(),
                self.payer.to_account_info(),
            ],
        )?;

        // Initialize the transfer fee extension data
        // This instruction must come before the instruction to initialize the mint data
        transfer_fee_initialize(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferFeeInitialize {
                    token_program_id: self.token_program.to_account_info(),
                    mint: self.mint_addr.to_account_info(),
                },
            ),
            Some(&self.payer.key()), // transfer fee config authority (update fee)
            Some(&self.payer.key()), // withdraw authority (withdraw fees)
            args.transfer_fee_basis_points, // transfer fee basis points (% fee per transfer)
            args.maximum_fee,        // maximum fee (maximum units of token per transfer)
        )?;

        // Initialize the standard mint account data
        initialize_mint2(
            CpiContext::new(
                self.token_program.to_account_info(),
                InitializeMint2 {
                    mint: self.mint_addr.to_account_info(),
                },
            ),
            9,                       // decimals
            &self.payer.key(),       // mint authority
            Some(&self.payer.key()), // freeze authority
        )?;

        self.check_mint_data()?;

        // Initialize token metadata
        self.initialize_token_metadata(args.name.clone(), args.symbol.clone(), args.uri.clone())?;

        // Ensure the mint account has the minimum balance required

        anchor_lang::solana_program::program::invoke(
            &instruction::create_associated_token_account(
                self.payer.key,
                self.sol_pool.key,
                self.mint_addr.key,
                self.token_program.key,
            ),
            &[
                self.payer.to_account_info(),
                self.token_pool.to_account_info(),
                self.sol_pool.to_account_info(),
                self.mint_addr.to_account_info(),
                self.associated_token_program.to_account_info(),
                self.system_program.to_account_info(),
                self.token_program.to_account_info(),
            ],
        )?;

        self.mint_tokens(SUPPLY)?;

        // Set the freeze authority for the mint
        self.set_freeze_authority()?;
        self.set_mint_authority()?;

        // Mint tokens to the token pool

        msg!(
            "Sent Create Fee to Fee Wallet : {} Sol ",
            ((self.global_configuration.create_pool_fee_lamports as f32) / (1_000_000_000 as f32))
        );

        // Transfer the creation fee to the fee account
        let _ = self.transfer_fee_to_fee_account()?;

        // Initialize bonding curve
        let _ = self.bonding_curve.init(
            self.global_configuration.initial_virtual_sol,
            self.global_configuration.initial_virtual_token,
            SUPPLY,
        );

        let _ = self.bonding_curve.update();

        Ok(())
    }
}
