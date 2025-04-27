/**
 * Simple JavaScript client for interacting with the User Registry Smart Contract
 * 
 * This example uses the Stellar SDK to interact with the deployed contract
 * on the Stellar blockchain.
 */

const StellarSdk = require('stellar-sdk');
const { Contract } = StellarSdk;
const { Server } = StellarSdk.SorobanRpc;

/**
 * User Registry Client
 */
class UserRegistryClient {
  /**
   * Create a new UserRegistryClient
   * 
   * @param {string} contractId - The contract ID of the deployed User Registry contract
   * @param {string} networkUrl - The URL of the Soroban RPC server (testnet or mainnet)
   * @param {string} sourceSecretKey - The secret key of the account to use for transactions
   */
  constructor(
    contractId,
    networkUrl = 'https://soroban-testnet.stellar.org:443',
    sourceSecretKey = null
  ) {
    this.server = new Server(networkUrl);
    this.contract = new Contract(contractId);
    
    if (sourceSecretKey) {
      this.source = StellarSdk.Keypair.fromSecret(sourceSecretKey);
    }
  }

  /**
   * Create a new user
   * 
   * @param {string} userId - The Stellar address of the user
   * @param {string} name - The user's name
   * @param {string} email - The user's email address
   * @param {Object} metadata - Additional user metadata as key-value pairs
   * @returns {Promise<boolean>} Promise resolving to a boolean indicating success or failure
   */
  async createUser(userId, name, email, metadata) {
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
   * @param {string} userId - The Stellar address of the user
   * @returns {Promise<Object|null>} Promise resolving to user data or null if not found
   */
  async getUser(userId) {
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
   * @param {string} userId - The Stellar address of the user
   * @param {string} name - The updated name
   * @param {string} email - The updated email
   * @param {Object} metadata - The updated metadata
   * @returns {Promise<boolean>} Promise resolving to a boolean indicating success or failure
   */
  async updateUser(userId, name, email, metadata) {
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
   * @param {string} userId - The Stellar address of the user to delete
   * @returns {Promise<boolean>} Promise resolving to a boolean indicating success or failure
   */
  async deleteUser(userId) {
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
   * 
   * @param {Object} metadata - The metadata object
   * @returns {Object} The converted metadata in contract format
   */
  convertMetadataToContractFormat(metadata) {
    const map = Object.entries(metadata).map(([key, value]) => ({
      key: { string: key },
      val: { string: value }
    }));
    
    return { map };
  }

  /**
   * Convert the contract's map format to a JavaScript object
   * 
   * @param {Object} contractMetadata - The metadata in contract format
   * @returns {Object} The converted metadata as a JavaScript object
   */
  convertMetadataFromContractFormat(contractMetadata) {
    const metadata = {};
    
    for (const entry of contractMetadata.map) {
      metadata[entry.key.string] = entry.val.string;
    }
    
    return metadata;
  }
}

// Example usage
async function runExample() {
  // Replace with your actual contract ID and network
  const client = new UserRegistryClient(
    'CONTRACT_ID_HERE',
    'https://soroban-testnet.stellar.org:443',
    'SECRET_KEY_HERE' // Optional: Your secret key for signing transactions
  );
  
  // Example: Creating a user
  console.log('Creating user...');
  const userId = 'GUSER_ADDRESS_HERE'; // Replace with actual Stellar address
  const createResult = await client.createUser(
    userId,
    'Jane Doe',
    'jane.doe@example.com',
    {
      account_type: 'standard',
      location: 'New York',
      age: '28',
      fitness_level: 'intermediate'
    }
  );
  
  console.log(`User creation ${createResult ? 'succeeded' : 'failed'}`);
  
  // Example: Getting user data
  console.log('\nRetrieving user data...');
  const userData = await client.getUser(userId);
  
  if (userData) {
    console.log('User found:');
    console.log(`Name: ${userData.name}`);
    console.log(`Email: ${userData.email}`);
    console.log('Metadata:');
    for (const [key, value] of Object.entries(userData.metadata)) {
      console.log(`  ${key}: ${value}`);
    }
  } else {
    console.log('User not found');
  }
  
  // Example: Updating a user
  console.log('\nUpdating user...');
  const updateResult = await client.updateUser(
    userId,
    'Jane Doe',
    'jane.doe@example.com',
    {
      account_type: 'premium',
      location: 'New York',
      age: '29',
      fitness_level: 'advanced',
      preferred_exercise_time: 'morning'
    }
  );
  
  console.log(`User update ${updateResult ? 'succeeded' : 'failed'}`);
  
  // Example: Getting updated user data
  console.log('\nRetrieving updated user data...');
  const updatedUserData = await client.getUser(userId);
  
  if (updatedUserData) {
    console.log('Updated user:');
    console.log(`Name: ${updatedUserData.name}`);
    console.log(`Email: ${updatedUserData.email}`);
    console.log('Metadata:');
    for (const [key, value] of Object.entries(updatedUserData.metadata)) {
      console.log(`  ${key}: ${value}`);
    }
  }
  
  // Example: Deleting a user (commented out for safety)
  /*
  console.log('\nDeleting user...');
  const deleteResult = await client.deleteUser(userId);
  console.log(`User deletion ${deleteResult ? 'succeeded' : 'failed'}`);
  
  // Verify deletion
  const deletedUserData = await client.getUser(userId);
  console.log('User data after deletion:', deletedUserData);
  */
}

// Uncomment to run the example
// runExample().catch(console.error); 