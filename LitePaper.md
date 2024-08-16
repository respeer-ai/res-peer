# ResPeer: Peer-to-Peer content publishing application on Linera

## TL;DR

ResPeer is a peer-to-peer content publishing application on Linera. It facilitates a creator ecosystem in which participants can earn the Linera native token (if any) and credits by creating content, selling artwork, reviewing, or participating in activities.

## Introduction

Linera is a new generation L1 blockchain which supports horizontal scaling by the concept called *microchains*. Validators in Linera could adjust their workers based on network demand, thus it could bring web2 level QoS to the web3 industry.

ResPeer is the first real-time social feed application on Linera. ResPeer aims to build a creator's community in which creators could monetize their work fairly in the web3 world.

Traditional monolithic chains pack transactions into blocks, then the blocks will be confirmed by validators to reach consensus. Due to all participants of the network competing for limited block space, some transactions have to wait for a long time to be processed.

Running a social feed application needs different resources: storage space for persistent messages, real-time notification for new content, scalability for burst access, etc. We have to leverage a lot of work to centralized servers, or integrate different chains for the mentioned requirements. In Linera, we can do all of these in one blockchain.

We can persist messages, content and images to blob storage which works as a Content Addressable Network, and stored contents will be replicated in the Linera network automatically. We can implement real-time notification with a native data stream. Most importantly, the message won't wait in the mempool any more, it could reach its target quickly. All of those are the basement to implement a web2-level web3 social feed application.

## Technique Overview

Basically, there are two types of producers in ResPeer: one is the content producer who publishes content; they will receive rewards from the reward pool of the foundation and credits as incentives issued by the credit application. The other is the asset producer who creates digital art and puts it on the market shelves for sale; the asset's price will be set in Linera.

An initial credit supply will be established when the application is deployed to Linera. The credits for each transaction will decay exponentially. Each credit amount earned by user interactions has its own lifetime and will be destroyed (or returned to the current total supply) when expired. If the credit balance is lower than the threshold, the total supply will be increased by 5% of the initial supply.

ResPeer provides a market for asset producers to showcase their work. Producers set the price of their work in Linera. The market has a unique setting that defines the exchange rate of credits and Linera tokens. When a user buys assets, if they have credits, they can pay for the work with Linera tokens and credits. A fixed ratio of the asset sale amount will be deposited into the foundation application, which will be used to reward the content creators and pay the fees for the reviewers.

ResPeer also implements a copilot to help creators improve their work with Linera Edge Generative AI. When a creator has a new task, they can select computing resources from the computing provider registry and then pay the provider for their task.

## Governance

As a web3 content application, ResPeer designs a Reviewer DAO that will review all content before publishing. If creators want to list their works in the ResPeer application, they should submit their works to the Reviewer DAO first. The initial members of the Reviewer DAO are configured when the Reviewer application is created. Community members can apply to be a reviewer based on their contributions to the community and their resumes. All types of applications will be reviewed by Reviewer DAO members in a distributed manner. If an application receives enough approvals, it will be approved automatically. The members of the Reviewer DAO can be removed by voting; thus, if some members exhibit disputed behavior, any Reviewer DAO member can propose to remove the disputed member, and the disputed account will be removed automatically after the proposal is approved.

## Creator Economy

### Feed Contents

Authors submit their content to the Reviewer DAO. If it's approved, the author will receive some credits according to the current balance of the credits. Moreover, if the foundation application has a Linera native token balance, it will distribute some tokens to the approved content author as remuneration.

### Avatar Marketplace

In ResPeer, every user can decorate their avatar with assets owned by them. It means they can design their avatar using some artwork they created, or use an NFT they bought from the avatar marketplace. Assets worn by the user will be displayed in ResPeer at different places, e.g., user center, content lists, content comments, etc. When users sell their work in the avatar marketplace, a fixed ratio of their transaction amount will be deposited into the foundation application.

### Reviewer DAO

If the foundation application has a balance, reviewers will receive Linera native tokens as their fee each time they approve or reject an application.

### Users Club

Above, we list how creators can earn tokens and credits in ResPeer. However, a social feed application must have readers. In ResPeer, readers can also earn tokens and credits by participating in activities held by the users' club. Anyone can apply to host an activity. If the activity application is approved by the Reviewer DAO, the foundation application will lock tokens for the activity. If the activity has items to vote for rewards, the voters for the winners will receive a part of the rewards according to the settings of the activity. The host of the activity will also earn some tokens and credits.

### Author Copilot and Computing Market

Authors may write some tiny mistakes in their content, or have great ideas but are poor at description. Thanks to Linera's Edge AI, we can have an AI copilot to help users with several tasks such as fixing grammar, rewriting, generating illustrations or covers, and generating abbreviations, etc. Community members can register their computing resources with the registry application at a price they think is reasonable. If the author thinks the price is acceptable, they will rent the computing resources for their tasks. Here, the author pays the computing provider before the task is finished. They don't need to pay for the full task in advance, but just pay for the finished part of the task. For example, if the rewriting needs to generate 10,000 words, the author will pay the computing provider for each 1,000 words. If the computing provider finds that the author has not paid for the upcoming part of the words, it won't generate the rest. Also, if the author finds that the paid part is not served, they won't pay anymore. The author may lose a very small amount of payment in that case.

### Foundation

A fixed ratio of each transaction amount will be deposited into the foundation application. Part of that will be used as the remuneration for the authors, the fees for reviewers, and the budget for activities.

## CheCko Wallet

Linera has a different account system from traditional blockchains. Signing blocks with a wallet client makes most of the existing wallets incompatible with Linera. ResPeer invented a **Microchain as a Service** architecture that separates chain storage and wallet clients. CheCko is the browser wallet client for Linera. It depends on the MaaS cluster to communicate with the Linera network. CheCko plans to implement the entire web3.js interface, so web applications that have already integrated Ethereum just need to replace their window.ethereum with window.linera to access the Linera network. The CheCko wallet serves as the login system for the ResPeer application. Other web applications that would like to integrate Linera can also connect to Linera using CheCko. In the future, the MaaS cluster may implement a web3.js compatible layer to allow existing wallets like MetaMask to access the Linera network if possible.

## Achievements

[Linera Developer Summer School 2023 First Place Winner](https://devpost.com/software/respeer-peer-to-peer-content-delivery-platform)
[Linera Autumn Developer School 2023 | Rust, WASM, Linera SDK First Place Winner](https://dorahacks.io/hackathon/linera-autumn-2023/results)
[Rebuild Ownership 2.0: Internet Privacy Linera Track First Place Winner](https://devfolio.co/projects/respeer-peertopeer-content-publishing-applicati-f0c9)
[Rust on Linera: Spring 2024 Hackathon First Place Winner](https://devpost.com/software/respeer-p2p-content-publishing-application-on-linera)
