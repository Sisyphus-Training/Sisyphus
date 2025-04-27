import { Contract } from 'stellar-sdk';
import { SorobanRpc } from '@stellar/ts-soroban-sdk';
import { Keypair } from 'stellar-sdk';

/**
 * Client for interacting with the User Registry smart contract
 */
export class UserRegistryClient {
  private contract: Contract;
  private server: SorobanRpc.Server;
  private source: Keypair;

  /**
   * Create a new UserRegistryClient
   * 
   * @param contractId - The contract ID of the deployed User Registry contract
   * @param networkUrl - The URL of the Soroban RPC server (testnet or mainnet)
   * @param sourceSecretKey - The secret key of the account to use for transactions
   */
  constructor(
    contractId: string,
    networkUrl: string = 'https://soroban-testnet.stellar.org:443',
    sourceSecretKey?: string
  ) {
    this.server = new SorobanRpc.Server(networkUrl);
    this.contract = new Contract(contractId);
    
    if (sourceSecretKey) {
      this.source = Keypair.fromSecret(sourceSecretKey);
    }
  }

  /**
   * Create a new user
   * 
   * @param userId - The Stellar address of the user
   * @param name - The user's name
   * @param email - The user's email address
   * @param metadata - Additional user metadata as key-value pairs
   * @returns Promise resolving to a boolean indicating success or failure
   */
  async createUser(
    userId: string,
    name: string,
    email: string,
    metadata: Record<string, string>
  ): Promise<boolean> {
    try {
      // Convert metadata to the format expected by the contract
      const metadataMap = this.convertMetadataToContractFormat(metadata);
      
      // Call the contract's create_user function
      const result = await this.contract.call(
        'create_user',
        userId,
        name,
        email,
        metadataMap
      );
      
      return result === true;
    } catch (error) {
      console.error('Error creating user:', error);
      return false;
    }
  }

  /**
   * Get user data
   * 
   * @param userId - The Stellar address of the user
   * @returns Promise resolving to user data or null if not found
   */
  async getUser(userId: string): Promise<{
    name: string;
    email: string;
    metadata: Record<string, string>;
  } | null> {
    try {
      const result = await this.contract.call(
        'get_user',
        userId
      );
      
      if (!result) {
        return null;
      }
      
      // Convert the contract result to a user-friendly format
      return {
        name: result.name,
        email: result.email,
        metadata: this.convertMetadataFromContractFormat(result.metadata)
      };
    } catch (error) {
      console.error('Error getting user:', error);
      return null;
    }
  }

  /**
   * Update an existing user
   * 
   * @param userId - The Stellar address of the user
   * @param name - The updated name
   * @param email - The updated email
   * @param metadata - The updated metadata
   * @returns Promise resolving to a boolean indicating success or failure
   */
  async updateUser(
    userId: string,
    name: string,
    email: string,
    metadata: Record<string, string>
  ): Promise<boolean> {
    try {
      // Convert metadata to the format expected by the contract
      const metadataMap = this.convertMetadataToContractFormat(metadata);
      
      // Call the contract's update_user function
      const result = await this.contract.call(
        'update_user',
        userId,
        name,
        email,
        metadataMap
      );
      
      return result === true;
    } catch (error) {
      console.error('Error updating user:', error);
      return false;
    }
  }

  /**
   * Delete a user
   * 
   * @param userId - The Stellar address of the user to delete
   * @returns Promise resolving to a boolean indicating success or failure
   */
  async deleteUser(userId: string): Promise<boolean> {
    try {
      // Call the contract's delete_user function
      const result = await this.contract.call(
        'delete_user',
        userId
      );
      
      return result === true;
    } catch (error) {
      console.error('Error deleting user:', error);
      return false;
    }
  }

  /**
   * Convert a JavaScript object to the contract's map format
   */
  private convertMetadataToContractFormat(metadata: Record<string, string>): any {
    const map: { key: { string: string }; val: { string: string } }[] = [];
    
    for (const [key, value] of Object.entries(metadata)) {
      map.push({
        key: { string: key },
        val: { string: value }
      });
    }
    
    return { map };
  }

  /**
   * Convert the contract's map format to a JavaScript object
   */
  private convertMetadataFromContractFormat(contractMetadata: any): Record<string, string> {
    const metadata: Record<string, string> = {};
    
    for (const entry of contractMetadata.map) {
      metadata[entry.key.string] = entry.val.string;
    }
    
    return metadata;
  }
}

// Example usage
async function exampleUsage() {
  // Create a client for the contract
  const client = new UserRegistryClient(
    'CONTRACT_ID_HERE',
    'https://soroban-testnet.stellar.org:443',
    'SECRET_KEY_HERE' // Your secret key for signing transactions
  );
  
  // Create a user
  const userId = 'GUSER_ADDRESS_HERE'; // User's Stellar address
  const createResult = await client.createUser(
    userId,
    'John Doe',
    'john.doe@example.com',
    {
      account_type: 'standard',
      preferred_exercises: 'walking,cycling',
      settings: 'default'
    }
  );
  
  console.log('User created:', createResult);
  
  // Get the user data
  const userData = await client.getUser(userId);
  console.log('User data:', userData);
  
  // Update the user
  const updateResult = await client.updateUser(
    userId,
    'John Doe',
    'john.doe@updated-email.com',
    {
      account_type: 'premium',
      preferred_exercises: 'walking,cycling,swimming',
      settings: 'customized'
    }
  );
  
  console.log('User updated:', updateResult);
  
  // Get the updated user data
  const updatedUserData = await client.getUser(userId);
  console.log('Updated user data:', updatedUserData);
}

// Call the example (uncomment to run)
// exampleUsage().catch(console.error); 