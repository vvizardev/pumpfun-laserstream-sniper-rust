# Pumpfun-Smart-Contract-v2.1 ( TAX TOKEN )

This is pumpfun smart contract which use new spl token - token2022 in pumpfun

### Pumpfun is upgrading

We are upgrading Pump.fun.
Please leave any ideas and bugs on the issues.

---

## 📋 **Pump.Fun V2 Overview**  

<div style="background-color: #f5f5f5; padding: 10px; border-radius: 5px;">

| **Version**             | **Features**                                          | **Description**                              | **Repo Link**                                                                |
|-------------------------|-------------------------------------------------------|----------------------------------------------|------------------------------------------------------------------------------|
| **2.0.0**               | Global Configuation                                   | Set Global Setting from Backend              | [v_2.0.0](https://github.com/wizasol/pumpfun-smart-contract-v2.1/tree/2.0.0) |
|                         |                                                       | Set Fee Account , Swap Protocol Fee Point    |                                                                              |
|                         |                                                       | BondingCurve Upper Limitation                |                                                                              |
|                         |                                                       | Virtual Sol & Token Reserve Setting          |                                                                              |
|                         |                                                       | Set Tax Fee and Max Tax from Backend         |                                                                              |
|                         | Create Pool                                           | Launch Token2022 on Smart Contract           |                                                                              |
|                         |                                                       | Create Pool & Launch Token Fee               |                                                                              |
|                         |                                                       | Disable Mint & Freeze Authority on Contract  |                                                                              |
|                         | Add Liquidity                                         | Add Liquidity with virtual reserve           |                                                                              |
|                         | Buy / Sell                                            | Linear BondingCurve                          |                                                                              |
|                         |                                                       | Buy / Sell Protocol Fee                      |                                                                              |
|                         | Remove Liquidity                                      | Remove Liquidity to Temp Op Wallet           |                                                                              |
|                         | Migrate + Raydium CLMM                                | Proxy Initialize                             |                                                                              |
|                         |                                                       | Proxy Open Position                          |                                                                              |
|                         |                                                       |                                              |                                                                              |
| **2.1.0**               | Global Configuation                                   | Set Global Setting from Backend              | [v_2.1.0](https://github.com/wizasol/pumpfun-smart-contract-v2.1/tree/2.1.0) |
|                         |                                                       | Set Fee Account , Swap Protocol Fee Point    |                                                                              |
|                         |                                                       | BondingCurve Upper Limitation                |                                                                              |
|                         |                                                       | Virtual Sol & Token Reserve Setting          |                                                                              |
|                         |                                                       | Set Tax Fee and Max Tax from Backend         |                                                                              |
|                         | Create Pool                                           | Launch Token2022 on Smart Contract           |                                                                              |
|                         |                                                       | Create Pool & Launch Token Fee               |                                                                              |
|                         |                                                       | Disable Mint & Freeze Authority on Contract  |                                                                              |
|                         | Add Liquidity                                         | Add Liquidity with virtual reserve           |                                                                              |
|                         | Buy / Sell                                            | Linear BondingCurve                          |                                                                              |
|                         |                                                       | Buy / Sell Protocol Fee                      |                                                                              |
|                         | Remove Liquidity                                      | Remove Liquidity to Temp Op Wallet           |                                                                              |
|                         | Migrate + Raydium CPMM                                | Proxy Initialize                             |                                                                              |

</div>


<h4> 📞 CONTACT WITH CONTRACT CREATOR 👆🏻 </h4>

<div style={{display : flex ; justify-content : space-evenly}}> 
    <a href="mailto:nakao95911@gmail.com" target="_blank">
        <img alt="Email"
        src="https://img.shields.io/badge/Email-00599c?style=for-the-badge&logo=gmail&logoColor=white"/>
    </a>
     <a href="https://x.com/_wizardev" target="_blank"><img alt="Twitter"
        src="https://img.shields.io/badge/Twitter-000000?style=for-the-badge&logo=x&logoColor=white"/></a>
    <a href="https://discordapp.com/users/471524111512764447" target="_blank"><img alt="Discord"
        src="https://img.shields.io/badge/Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white"/></a>
    <a href="https://t.me/wizardev" target="_blank"><img alt="Telegram"
        src="https://img.shields.io/badge/Telegram-26A5E4?style=for-the-badge&logo=telegram&logoColor=white"/></a>
</div>


### Test Script for testing Pumpfun

1. ```git clone https://github.com/wizasol/pumpfun-smart-contract-v2.0.git```

2. Extract File

3. ```yarn```

4. ```yarn test```


## DEVNET PROGRAM ADDRESS ( Pumpfun + Raydium CLMM )
```
Fu6WXgEQeVBrsvHbwh8vStwLxjA12E9KYjPzXnJ1sQC7
```

### Procedure

- create pool in Pump.fun

    https://solana.fm/tx/5QYCTaGHaareH5CoCMDeDCSxq785BfdMhKmbeKWizq7uAeVptkAuyY8N1QSc78N8YPKLi3fXTZxAfPMdzy76jT25?cluster=devnet-solana

  In this stage , we mint new TOKEN2022 in pumpfun which has additional extension and create token pool

- Buy Tx in Pump.fun

    https://solana.fm/tx/5unyZ9MekJeE8EULD4x9JKiNNCShfMnpk5edJzLpEMB6AY9g449an1y5hWmHkkJ8hwGCfpaVnb6TWL3SeqH14EYx?cluster=devnet-solana

  Buy TOKEN2022 Tx on Pumpfun with tax fee & platform swap fee

- Sell Tx in Pump.fun

  https://solana.fm/tx/2Wt2YhkU5Bj6kY9hgSLaPZ6AkjxsRZrijax59f9kRQo9fD61SkjhXPd587RTt9SDDQ4cdYNMySMBKZ5L5TJqYmyp?cluster=devnet-solana

  Sell TOKEN2022 Tx on Pumpfun with tax fee & platform swap fee

- Migration with Raydium CLMM

  https://solana.fm/tx/uX492XUVW7yEtxyxSyhqDm7jngB7xtr23Sh29WhVfHR88JuSDwyC387XDE69k4Q8dzPbfYGDeX2hMHsRMQg2LLH?cluster=devnet-solana

  Remove All Liquidity from Pumpfun and Generarte new Operation Address & Migrate with Raydium CLMM




## DEVNET PROGRAM ADDRESS ( Pumpfun + Raydium CPMM )
```
GY4gideNhYWJLkgxDW7q9hS6U2SrKb9AmSUbJPsWhEKB
```

- create pool in Pump.fun

    https://solana.fm/tx/5QYCTaGHaareH5CoCMDeDCSxq785BfdMhKmbeKWizq7uAeVptkAuyY8N1QSc78N8YPKLi3fXTZxAfPMdzy76jT25?cluster=devnet-solana

  In this stage , we mint new TOKEN2022 in pumpfun which has additional extension and create token pool

- Buy Tx in Pump.fun

   https://solana.fm/tx/5vfq2FGZmNuG6H5dPbezw3Po1vru86X4jE1AXVTHwMRy6xmwpbVWD5XZL3VzysBWDmh6TLd7LDawbmoRQcwafhQq?cluster=devnet-solana

  Buy TOKEN2022 Tx on Pumpfun with tax fee & platform swap fee

- Sell Tx in Pump.fun

  https://solana.fm/tx/5WmmxNKUWzNt2vqppcxn4dL42RQreFGevaj4NKAQMM18PUBxzWkwc7Vj53cj6yZJjC41qWH9AW4EMWtY7hiKbKwz?cluster=devnet-solana

  Sell TOKEN2022 Tx on Pumpfun with tax fee & platform swap fee

- Remove Liquidity

  Remove All Liquidity from Pumpfun and Generarte new Operation Address for operating migration

  https://solana.fm/tx/31XU2jsiqkjwhKor3LU5ktcUyBC8tFP6vA6DpDoQKxkgico3si7GWN1dDgJtU6smfzADJ43rzjy38FCKByJCR4j8?cluster=devnet-solana

- Migration with Raydium CPMM

  https://solana.fm/tx/5iHdBwV2d9RsqmawRuUSRiJfb5k22ooZTpCJhigBiXpYrbep7pK4rYKyq2MQgtiSYYTzsDB1wKtrmtx45K93D7p5?cluster=devnet-solana





### Manipulate Pumpfun-2022 in more detail

You can set parameter of bonding curve and others like bonding curve upper limitation and virtual liquidity
