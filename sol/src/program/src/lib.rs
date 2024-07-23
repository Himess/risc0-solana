use groth16_solana::decompression::{decompress_g1, decompress_g2};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use std::convert::TryInto;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let compressed_proof_a: &[u8; 32] = instruction_data[0..32]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let compressed_proof_b: &[u8; 64] = instruction_data[32..96]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let compressed_proof_c: &[u8; 32] = instruction_data[96..128]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let proof_a = decompress_g1(&compressed_proof_a).map_err(|_| ProgramError::Custom(1))?;
    let proof_b = decompress_g2(&compressed_proof_b).map_err(|_| ProgramError::Custom(2))?;
    let proof_c = decompress_g1(&compressed_proof_c).map_err(|_| ProgramError::Custom(3))?;

    let public_inputs = PUBLIC_INPUTS;
    let vk = VERIFYING_KEY;
    let mut verifier = Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &public_inputs, &vk)
        .map_err(|_| ProgramError::Custom(4))?;

    let status = verifier.verify().map_err(|_| {
        #[cfg(target_os = "solana")]
        {
            use solana_program::msg;
            msg!("Proof verification failed");
            msg!("Public inputs: {:?}", public_inputs);
            msg!("Proof A: {:?}", proof_a);
            msg!("Proof B: {:?}", proof_b);
            msg!("Proof C: {:?}", proof_c);
        }
        ProgramError::Custom(5)
    })?;

    if status {
        #[cfg(target_os = "solana")]
        {
            use solana_program::msg;
            msg!("Proof verification sucessful!");
        }
    }

    Ok(())
}

pub const VERIFYING_KEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 10,

    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_ic: &[
        [
            3, 183, 175, 189, 219, 73, 183, 28, 132, 200, 83, 8, 65, 22, 184, 81, 82, 36, 181, 186,
            25, 216, 234, 25, 151, 2, 235, 194, 13, 223, 32, 145, 15, 37, 113, 122, 93, 59, 91, 25,
            236, 104, 227, 238, 58, 154, 67, 250, 186, 91, 93, 141, 18, 241, 150, 59, 202, 48, 179,
            1, 53, 207, 155, 199,
        ],
        [
            46, 253, 85, 84, 166, 240, 71, 175, 111, 174, 244, 62, 87, 96, 235, 196, 208, 85, 186,
            47, 163, 237, 53, 204, 176, 190, 62, 201, 189, 216, 132, 71, 6, 91, 228, 97, 74, 5, 0,
            255, 147, 113, 161, 152, 238, 177, 78, 81, 111, 13, 142, 220, 24, 133, 27, 149, 66,
            115, 34, 87, 224, 237, 44, 162,
        ],
        [
            29, 157, 232, 254, 238, 178, 82, 15, 152, 205, 175, 129, 90, 108, 114, 60, 82, 162, 37,
            234, 115, 69, 191, 125, 212, 85, 176, 176, 113, 41, 23, 84, 8, 229, 196, 41, 191, 243,
            112, 105, 166, 75, 113, 160, 140, 34, 139, 179, 53, 180, 245, 195, 5, 24, 42, 18, 82,
            60, 173, 192, 67, 149, 211, 250,
        ],
        [
            18, 4, 92, 105, 55, 33, 222, 133, 144, 185, 99, 131, 167, 143, 52, 120, 44, 79, 164,
            63, 119, 223, 199, 154, 26, 86, 22, 208, 50, 53, 159, 65, 14, 171, 53, 159, 255, 133,
            91, 30, 162, 209, 152, 18, 251, 112, 105, 90, 65, 234, 44, 4, 42, 173, 31, 230, 229,
            137, 177, 112, 241, 142, 62, 176,
        ],
        [
            13, 117, 56, 250, 131, 38, 119, 205, 221, 228, 32, 185, 236, 82, 102, 29, 198, 53, 117,
            151, 19, 10, 255, 211, 41, 210, 72, 221, 79, 107, 251, 150, 35, 187, 30, 32, 198, 17,
            220, 4, 68, 10, 71, 51, 31, 169, 4, 174, 10, 38, 227, 229, 193, 129, 150, 76, 94, 224,
            182, 13, 166, 65, 175, 89,
        ],
        [
            21, 167, 160, 214, 213, 132, 208, 197, 115, 195, 129, 111, 129, 38, 56, 52, 41, 57, 72,
            249, 50, 187, 184, 49, 240, 228, 142, 147, 187, 96, 96, 102, 34, 163, 43, 218, 199,
            187, 250, 245, 119, 151, 237, 67, 231, 70, 236, 67, 157, 181, 216, 174, 25, 82, 120,
            255, 191, 89, 230, 165, 179, 241, 188, 218,
        ],
        [
            4, 136, 219, 130, 55, 89, 21, 224, 41, 30, 53, 234, 66, 160, 129, 174, 154, 139, 151,
            33, 163, 221, 150, 192, 171, 102, 241, 161, 48, 130, 31, 175, 6, 47, 176, 127, 13, 8,
            36, 228, 239, 219, 6, 158, 22, 31, 22, 162, 91, 196, 132, 188, 156, 228, 30, 1, 178,
            246, 197, 186, 236, 249, 236, 147,
        ],
        [
            9, 41, 120, 80, 67, 24, 240, 221, 136, 156, 137, 182, 168, 17, 176, 118, 119, 72, 170,
            188, 227, 31, 15, 22, 252, 37, 198, 154, 195, 163, 64, 125, 37, 211, 235, 67, 249, 133,
            45, 90, 162, 9, 173, 19, 80, 154, 208, 173, 221, 203, 206, 254, 81, 197, 104, 26, 177,
            78, 86, 210, 51, 116, 60, 87,
        ],
        [
            3, 41, 86, 208, 125, 147, 53, 187, 213, 220, 195, 141, 216, 40, 92, 137, 70, 210, 168,
            103, 105, 236, 85, 37, 165, 209, 246, 75, 122, 251, 75, 93, 28, 108, 154, 181, 15, 16,
            35, 88, 65, 211, 8, 11, 123, 84, 185, 187, 184, 1, 83, 141, 67, 46, 241, 222, 232, 135,
            59, 44, 152, 217, 237, 106,
        ],
        [
            34, 98, 189, 118, 119, 197, 102, 193, 36, 150, 200, 143, 226, 60, 0, 239, 21, 40, 5,
            156, 73, 7, 247, 14, 249, 157, 2, 241, 181, 208, 144, 0, 34, 45, 86, 133, 116, 53, 235,
            160, 107, 36, 195, 125, 122, 10, 206, 88, 85, 166, 62, 150, 65, 159, 130, 7, 255, 224,
            227, 229, 206, 138, 68, 71,
        ],
    ],
};

pub const PUBLIC_INPUTS: [[u8; 32]; 9] = [
    [
        34, 238, 251, 182, 234, 248, 214, 189, 46, 67, 42, 25, 71, 58, 145, 58, 61, 28, 116, 110,
        60, 17, 82, 149, 178, 187, 160, 211, 37, 226, 174, 231,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51,
        152, 17, 147,
    ],
    [
        4, 247, 199, 87, 230, 85, 103, 90, 28, 183, 95, 100, 200, 46, 3, 158, 247, 196, 173, 146,
        207, 167, 108, 33, 199, 18, 13, 204, 198, 101, 223, 186,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 49,
        65, 41,
    ],
    [
        7, 130, 55, 65, 197, 232, 175, 217, 44, 151, 149, 225, 75, 86, 158, 105, 43, 229, 65, 87,
        51, 150, 168, 243, 176, 175, 11, 203, 180, 149, 72, 103,
    ],
    [
        46, 93, 177, 62, 42, 66, 223, 153, 51, 193, 146, 49, 154, 41, 69, 198, 224, 13, 87, 80,
        222, 171, 37, 141, 0, 1, 50, 172, 18, 28, 213, 213,
    ],
    [
        40, 141, 45, 3, 180, 200, 250, 112, 108, 94, 35, 143, 82, 63, 125, 9, 147, 37, 191, 75, 62,
        221, 138, 20, 166, 151, 219, 237, 254, 58, 230, 189,
    ],
    [
        33, 100, 143, 241, 11, 251, 73, 141, 229, 57, 129, 168, 83, 23, 235, 147, 138, 225, 177,
        250, 13, 97, 226, 162, 6, 232, 52, 95, 128, 84, 90, 202,
    ],
    [
        25, 178, 1, 208, 219, 169, 222, 123, 113, 202, 165, 77, 183, 98, 103, 237, 187, 93, 178,
        95, 169, 156, 38, 100, 125, 218, 104, 94, 104, 119, 13, 21,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

    use solana_program::pubkey::Pubkey;
    use std::fs;
    use std::ops::Neg;

    type G1 = ark_bn254::g1::G1Affine;
    type G2 = ark_bn254::g2::G2Affine;

    #[test]
    fn test_process_instruction() {
        let compressed_proof = fs::read("../client/compressed_proof.bin")
            .expect("Failed to read compressed proof file");

        assert_eq!(compressed_proof.len(), 128);

        let accounts: [AccountInfo; 0] = [];

        let result = process_instruction(&Pubkey::default(), &accounts, &compressed_proof);

        assert!(result.is_ok());
    }

    fn change_endianness(bytes: &[u8]) -> Vec<u8> {
        let mut vec = Vec::new();
        for b in bytes.chunks(32) {
            for byte in b.iter().rev() {
                vec.push(*byte);
            }
        }
        vec
    }

    #[test]
    fn proof_verification_should_succeed() {
        let proof = fs::read("../client/proof.bin").expect("Failed to read compressed proof file");

        let proof_a: G1 = G1::deserialize_with_mode(
            &*[&change_endianness(&proof[0..64]), &[0u8][..]].concat(),
            Compress::No,
            Validate::Yes,
        )
        .unwrap();
        let mut proof_a_neg = [0u8; 65];
        proof_a
            .neg()
            .x
            .serialize_with_mode(&mut proof_a_neg[..32], Compress::No)
            .unwrap();
        proof_a
            .neg()
            .y
            .serialize_with_mode(&mut proof_a_neg[32..], Compress::No)
            .unwrap();

        let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
        let proof_b = proof[64..192].try_into().unwrap();
        let proof_c = proof[192..256].try_into().unwrap();

        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &PUBLIC_INPUTS, &VERIFYING_KEY)
                .unwrap();
        verifier.verify().unwrap();
    }

    #[test]
    fn proof_verification_with_compressed_inputs_should_succeed() {
        let mut public_inputs_vec = Vec::new();
        for input in PUBLIC_INPUTS.chunks(32) {
            public_inputs_vec.push(input);
        }

        let compressed_proof = fs::read("../client/compressed_proof.bin")
            .expect("Failed to read compressed proof file");
        let compressed_proof_a: &[u8; 32] = compressed_proof[0..32].try_into().unwrap();
        let compressed_proof_b: &[u8; 64] = compressed_proof[32..96].try_into().unwrap();
        let compressed_proof_c: &[u8; 32] = compressed_proof[96..128].try_into().unwrap();
        //
        let proof_a = decompress_g1(&compressed_proof_a).unwrap();
        let proof_b = decompress_g2(&compressed_proof_b).unwrap();
        let proof_c = decompress_g1(&compressed_proof_c).unwrap();
        let mut verifier =
            Groth16Verifier::new(&proof_a, &proof_b, &proof_c, &PUBLIC_INPUTS, &VERIFYING_KEY)
                .unwrap();
        verifier.verify().unwrap();
    }
}
