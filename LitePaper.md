# ResPeer: Peer-to-Peer content publishing application on Linera

## TL;DR

ResPeer is a Peer-to-Peer content publishing application on Linera. It facilitates a creator ecosystem which participants can earn Linera native token (if any) and credits by creating contents, selling art works, reviewing or joining in activities.

## Introduction

Linera is new generation L1 blockchain which supports horizontal scaling by the concept called *microchain*. Validators in Linera could adjust their workers based on network demand, thus it could bring web2 level QoS to web3 industry. ResPeer is the first real time social feed application on Linera. ResPeer aims to build a creators' community in which creators could monitize their work fairly in web3 world. Tranditional monolithic chain packs transactions into blocks, then the blocks will be confirmed by validators to reach consensus. Due to all participants of the network compete for limited block space, some transactions have to wait for long time to be processed.  Running a social feed application needs different resources: storage space for persistent messages, real time notification for new contents, scalability for burst access, etc. We have to leverage a lot of work to centralized servers, or integrate different chains for the metioned requirements. In Linera, we can do these all in one blockchain. We can persistent messages, contents and images to blob storage which works as a Content Addressable Network, stored contents will be replicated in Linera network automatically. We can implement real time notification with native data stream. Most importantly, the message won't wait in mempool any more, it could reach its target quickly. All of those are the basement to implement a web2 level web3 social feed application.

## Technique Overview

Basically, there're two types producer in ResPeer, one is the content producer who publishes content, they will get rewards from the reward pool of the fundation, and get credits incentive issued by credit application. Another is asset producer who create digital arts then put on shelves of the market for sale, the asset's price will be set with Linera.

A initial credit supply will be set when the application is deploy to Linera. The credits of each reaction will be exponential decay. Each credit amount earned by user reaction has its alive time, and will be destroyed (or return to current total supply) when expired. If the credit balance is lower than threshold, the total supply will be increased with 5% of the initial supply.

ResPeer provides a market for asset producers to put their work on shelves. Producer sets price of their work with Linera. The market have a unique setting which define the exchange rate of the credits and Linera token. When user buy assets, if they have credits, they can pay to the work with Linera token and credits. A fix ratio of the asset sale amount will be deposit to fundation application, which will be used to reward the content, and pay the fee for the reviewer.

ResPeer also implements a copilot the help creators improve their work with Linera Edge Generative AI. When creator has new task, they can select computing resource from computing provider registry, then pay to the provider for their task.

## Governance

As a web3 content application, ResPeer design a reviewer DAO which will review all content before publishing. If creators want to list their works in ResPeer application, they should submit their works to reviewer DAO firstly. The initial members of the reviewer DAO is configured when the reviewer application is created. Community members could apply to be a reviewer with their contribution to the community and their resume. All types applications will be reviewed by reviewer DAO members distributely. If an application get enough approvals, it'll be approved automatically. The members of the reviewer DAO could be removed by voting, thus if some members have dispute behavior, any reviewer DAO member could propose to remove the dispute member, then the dispute account will be removed automatically after the proposal is approved.

## Creator Economy

### Feed Contents

Author submit their content to reviewer DAO. If it's approved, author will get some credits according to current balance of the credits. Moreover, if foundation application has Linera native token balance, it'll will distribute some tokens to the approved content author as remuneration.

### Avatar Marketplace

### Reviewer DAO

### Users Club

### Author Copilot and Computing Market

### Foundation

## Achievements
