# Exercise Routine Builder for Physical Therapists

## Overview

The Sisyphus Exercise Routine Builder is a specialized tool designed for physical therapists to streamline the process of creating customized exercise routines. By taking into account individual patient data, personal preferences, objectives, and specific health considerations, this tool aims to enhance patient engagement and support effective rehabilitation.

## Features

- **Personalized Exercise Plans**: Generate tailored exercise routines based on individual patient profiles.
- **Patient Data Integration**: Input and access detailed patient information including age, health status, and activity levels.
- **Preference Mapping**: Incorporate patient preferences to ensure compliance and enjoyment in their exercise regimen.
- **Goal Setting**: Enable patients and therapists to set and track personal fitness and rehabilitation goals.
- **Progress Tracking**: Monitor patient progress over time with built-in analytics and reporting features.
- **Exercise Library**: Access a comprehensive library of exercises categorized by difficulty, type, and target area.

## Benefits

- **Improved Patient Outcomes**: Personalized routines lead to better adherence and results.
- **Time Efficiency**: Save time in routine development, allowing therapists to focus more on patient care.
- **Enhanced Communication**: Streamlined sharing of exercise plans between therapists and patients.
- **Adaptability**: Easily adjust routines based on ongoing assessments and feedback.

## Getting Started

### Prerequisites

- Access to patient data (with appropriate privacy considerations).
- Basic understanding of physical therapy principles.

### Installation

1. Download the latest version of the Exercise Routine Builder.
2. Follow the installation instructions provided in the setup guide.

### Usage

1. **Create Patient Profiles**: Enter patient information including health conditions, age, and activity levels.
2. **Set Personal Objectives**: Collaborate with patients to define their fitness goals.
3. **Build Routines**: Use the tool to select appropriate exercises and generate a routine.
4. **Track Progress**: Regularly update patient progress and adjust routines as necessary.

# Exercise Routine Builder Roadmap

The roadmap outlines the planned features, enhancements, and milestones for the Exercise Routine Builder tool. It serves as a guide for the development team, stakeholders, and users to understand the direction of the project and upcoming improvements.

## Getting Started

To get a local copy up and running, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/Sisyphus.git
   ```
2. Navigate to the project directory:
   ```bash
   cd Sisyphus
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Run the development server:
   ```bash
   npm run dev
   ```
5. Your application will be available at [http://localhost:3000](http://localhost:3000).

## Roadmap Overview

### Q1 2025: Initial Launch

- **Feature Completion:**
  - Finalize core features including patient data integration, personalized exercise plans, and progress tracking.
  - Develop a comprehensive exercise library with categorization.

- **User Testing:**
  - Conduct beta testing with a select group of physical therapists to gather feedback.
  - Address any critical bugs and usability issues.

- **Documentation:**
  - Complete user manuals and setup guides.
  - Prepare FAQs and troubleshooting resources.

### Q2 2025: Enhanced Functionality

- **Goal Setting Module:**
  - Implement a feature for setting and tracking patient-specific fitness and rehabilitation goals.

- **Customization Options:**
  - Allow therapists to customize exercise routines further based on individual patient needs.

- **Mobile Compatibility:**
  - Develop a mobile-responsive version of the tool for use on tablets and smartphones.

### Q3 2025: User Engagement

- **Progress Analytics:**
  - Introduce advanced analytics for tracking patient progress over time, including visual reports and insights.

- **Feedback Mechanism:**
  - Implement a system for patients to provide feedback on exercises and routines.

- **Patient Portal:**
  - Develop a patient-facing portal where patients can view their routines, track progress, and communicate with their therapist.

### Q4 2025: Community and Collaboration

- **Collaboration Features:**
  - Enable features for therapists to collaborate and share routines with colleagues.

- **Community Forum:**
  - Launch a community forum for therapists to discuss best practices, share tips, and provide support.

### 2026 and Beyond: Continuous Improvement

- **Integration with Wearable Devices:**
  - Explore integration with fitness trackers and wearable devices for real-time data collection and monitoring.

- **Artificial Intelligence Enhancements:**
  - Investigate the use of AI to suggest modifications to routines based on patient progress and feedback.

- **Expansion of Exercise Library:**
  - Continuously add new exercises and routines based on emerging research and therapeutic practices.

- **Regular Software Updates:**
  - Maintain a regular schedule for software updates to enhance functionality and security.

## Conclusion

This roadmap is a living document that will evolve based on user feedback, technological advancements, and the changing needs of physical therapists and their patients. Your input is invaluable in shaping the future of the Exercise Routine Builder. Thank you for your continued support and collaboration!

## Contribution

We welcome contributions to enhance the functionality and usability of the Exercise Routine Builder. Please fork the repository and submit a pull request with your proposed changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

For support or inquiries, please reach out to [Luisfe.vera@gmail.com](mailto:luisfe.vera@gmail.com).

---

Thank you for using the Exercise Routine Builder! Together, we can make rehabilitation more effective and engaging for every patient.

# Patient-Trainer Link Smart Contract

A Stellar smart contract built with Soroban that establishes secure links between patients and personal trainers, enabling data sharing and management.

## Features

- **Trainer Registration**: Personal trainers can register on the platform
- **Patient Linking**: Trainers can link patients to their account using patient addresses
- **Data Management**: Trainers can update patient-specific data including:
  - Exercise routines
  - Meal plans
  - Progress updates
- **Access Control**: Patients can only access their own data, trainers can only update their linked patients
- **Event Logging**: All major actions emit events for monitoring

## Contract Structure

### Data Types

- `PatientData`: Stores patient information including routines, meal plans, and progress
- `TrainerInfo`: Stores trainer information and patient count
- `DataKey`: Enum for different storage keys

### Main Functions

1. `initialize()`: Initialize the contract
2. `register_trainer()`: Register a new trainer
3. `link_patient()`: Link a patient to a trainer
4. `update_exercise_routines()`: Update patient's exercise routines
5. `update_meal_plans()`: Update patient's meal plans
6. `update_progress()`: Update patient's progress
7. `get_patient_data()`: Retrieve patient data (requires patient auth)
8. `get_trainer_info()`: Get trainer information
9. `get_patient_trainer()`: Get the trainer linked to a patient
10. `unlink_patient()`: Remove patient-trainer link

## Deployment

### Prerequisites

- Rust and Cargo installed
- Soroban CLI installed
- Stellar testnet account with funds

### Build and Deploy

```bash
# Build the contract
cd contracts/trainer_patient_link
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/trainer_patient_link.wasm

# Deploy to testnet
./scripts/deploy.sh
```

### Testing

Run unit tests:
```bash
cargo test
```

Run integration tests on testnet:
```bash
./scripts/test_interactions.sh
```

## Usage Examples

### Register as a Trainer

```bash
soroban contract invoke \
    --id CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    register_trainer \
    --trainer_id TRAINER_ADDRESS
```

### Link a Patient

```bash
soroban contract invoke \
    --id CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    link_patient \
    --trainer_id TRAINER_ADDRESS \
    --patient_id PATIENT_ADDRESS
```

### Update Patient Data

```bash
soroban contract invoke \
    --id CONTRACT_ID \
    --source trainer \
    --network testnet \
    -- \
    update_exercise_routines \
    --trainer_id TRAINER_ADDRESS \
    --patient_id PATIENT_ADDRESS \
    --routines '["Exercise 1", "Exercise 2"]'
```

### Patient Access Data

```bash
soroban contract invoke \
    --id CONTRACT_ID \
    --source patient \
    --network testnet \
    -- \
    get_patient_data \
    --patient_id PATIENT_ADDRESS
```

## Security Considerations

1. **Authentication**: All sensitive operations require authentication from the appropriate party
2. **Authorization**: Only linked trainers can update patient data
3. **Data Privacy**: Patients can only access their own data
4. **Error Handling**: Comprehensive error messages for debugging while maintaining security

## Error Codes

- `exists`: Trainer already registered
- `no_trainer`: Trainer not found
- `inactive`: Trainer account is inactive
- `linked`: Patient already linked to a trainer
- `no_data`: Patient data not found
- `no_link`: No link exists between patient and trainer
- `wrong_trainer`: Trainer is not authorized for this patient
- `not_linked`: Patient-trainer link not found

## Future Enhancements

- Notification system for data updates
- Multi-trainer support for patients
- Data archival and history tracking
- Integration with health monitoring devices
- Export functionality for patient data
