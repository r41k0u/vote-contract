use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod votes {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let chair = &mut ctx.accounts.chair;
        chair.chairperson = *ctx.accounts.chairP.key;
        Ok(())
    }

    pub fn add_voter(ctx: Context<NewVoter>, weight: u32) -> ProgramResult {
        let acc = &mut ctx.accounts.voter_acc;
        acc.address = *ctx.accounts.voter.key;
        acc.weight = weight;
        acc.voted = false;
        Ok(())
    }

    pub fn add_new_proposal(ctx: Context<NewProposal>, id: String) -> ProgramResult {
        let prop = &mut ctx.accounts.prop_acc;
        prop.name = id;
        prop.votes = 0;
        prop.to_account_info().owner = ctx.accounts.proposalda.key;
        Ok(())
    }

    pub fn delegate(ctx: Context<DelegateVotes>) -> ProgramResult {
        ctx.accounts.delegate_account.weight += ctx.accounts.vote_account.weight;
        ctx.accounts.vote_account.voted = true;
        Ok(())
    }

    pub fn vote(ctx: Context<GiveVote>) -> ProgramResult {
        ctx.accounts.prop_acc.votes += ctx.accounts.vote_account.weight;
        ctx.accounts.vote_account.voted = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=chairP)]
    pub chair: Account<'info, Chairperson>,
    #[account(mut)]
    pub chairP: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewVoter<'info> {
    #[account(mut)]
    pub chairperson: Signer<'info>,
    pub voter: AccountInfo<'info>,
    #[account(has_one = chairperson)]
    pub chair: Account<'info, Chairperson>,
    #[account(init, payer = chairperson)]
    pub voter_acc: Account<'info, Voter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NewProposal<'info> {
    #[account(mut)]
    pub chairperson: Signer<'info>,
    #[account(seeds=[b"proposal"], bump)]
    pub proposalda: AccountInfo<'info>,
    #[account(has_one = chairperson)]
    pub chair: Account<'info, Chairperson>,   
    #[account(init, payer = chairperson)]
    pub prop_acc: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DelegateVotes<'info> {
    #[account(mut)]
    pub address: Signer<'info>,
    #[account(mut, has_one=address, constraint = !vote_account.voted)]
    pub vote_account: Account<'info, Voter>,
    #[account(mut, constraint = !delegate_account.voted)]
    pub delegate_account: Account<'info, Voter>,
}

#[derive(Accounts)]
pub struct GiveVote<'info> {
    #[account(mut)]
    pub address: Signer<'info>,
    #[account(mut, has_one = address, constraint = !vote_account.voted)]
    pub vote_account: Account<'info, Voter>,
    #[account(mut)]
    pub prop_acc: Account<'info, Proposal>,
}

#[account]
#[derive(Default)]
pub struct Voter {
    weight: u32,
    voted: bool,
    address: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Proposal {
    name: String,
    votes: u32,
}

#[account]
#[derive(Default)]
pub struct Chairperson {
    chairperson: Pubkey,
}