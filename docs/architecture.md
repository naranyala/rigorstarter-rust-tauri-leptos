# System Architecture

This project implements a high-performance desktop application using a tiered architecture that separates domain logic from the delivery mechanism.

## Architecture Pattern: MVVM

The application follows a modified Model-View-ViewModel (MVVM) pattern to ensure scalability and maintainability.

### 1. Model (Core)
Located in `src/core/`, the model layer contains:
- **Shared Models**: Data structures (DTOs) used across the application.
- **Core Logic**: Pure functions for business rules (e.g., registry filtering and sorting). This layer has zero dependencies on UI frameworks or system APIs.

### 2. ViewModel (Services)
Located in `src/services/`, the service layer acts as the ViewModel:
- **State Management**: Uses Leptos signals to maintain application state.
- **Dependency Injection**: Services are provided via Leptos Context, allowing components to access shared state without prop-drilling.
- **Orchestration**: Coordinates between the Core logic and the Tauri backend.

### 3. View (Components)
Located in `src/components/`, the view layer is responsible for:
- **Reactive UI**: Rendering components based on the state provided by services.
- **Event Handling**: Triggering service methods in response to user interaction.

## Communication Layer

The application uses a bridge pattern for Frontend-Backend communication:

- **Frontend**: Invokes Tauri commands via the `invoke` API.
- **Backend**: Processes requests in native Rust, interacting with the OS, and returns serialized JSON.
- **Shared Contracts**: Both ends agree on the data structures defined in the Core models.

## Component Registry

A central registry system decouples UI labels from system identifiers. This allows for dynamic discovery of components and utilities without hardcoding paths in the UI components.
