export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

function numberToHash(value: number): Hash | null {
	switch (value) {
		case 0:
			return Hash.KECCAK256;
		case 1:
			return Hash.SHA256;
		default:
			return null; // Return null or throw an error for invalid values
	}
}
