use core::mem::size_of;

use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError,
    pubkey::find_program_address,
};
use pinocchio_secp256r1_instruction::{SECP256R1_COMPRESSED_PUBKEY_LENGTH, Secp256r1Pubkey};
use pinocchio_system::instructions::Transfer;

pub struct DepositAccounts<'a> {
    pub payer: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, vault, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Accounts Checks
        if !payer.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        // Return the accounts
        Ok(Self {
            payer,
            vault,
            system_program,
        })
    }
}

pub struct DepositInstructionData {
    pub pubkey: Secp256r1Pubkey,
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<Secp256r1Pubkey>() + size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }

        let pubkey =
            Secp256r1Pubkey::try_from(&data[0..SECP256R1_COMPRESSED_PUBKEY_LENGTH]).unwrap();

        let amount = u64::from_le_bytes(
            data[SECP256R1_COMPRESSED_PUBKEY_LENGTH
                ..SECP256R1_COMPRESSED_PUBKEY_LENGTH + size_of::<u64>()]
                .try_into()
                .unwrap(),
        );

        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self { pubkey, amount })
    }
}

pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        // Check vault address
        let (vault_key, _) = find_program_address(
            &[
                b"vault",
                &self.instruction_data.pubkey[..1],
                &self.instruction_data.pubkey[1..33],
            ],
            &crate::ID,
        );
        if vault_key.ne(self.accounts.vault.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Transfer {
            from: self.accounts.payer,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount,
        }
        .invoke()?;

        Ok(())
    }
}