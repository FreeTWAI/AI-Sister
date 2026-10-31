# API Reference

This document provides a high-level overview of the core modules in AI-Sister and their responsibilities.

## Core Crates

### `sister-core`
The platform-agnostic core of AI-Sister. It handles memory, database storage, privacy checks, and logic execution without any knowledge of the underlying OS (Windows, macOS, or Linux).

- **`model`**: Contains core data structures such as `Signal`, `FrameCapture`, `OcrBlock`, and events.
- **`db`**: Manages the SQLite database operations, including append-only storage and Full-Text Search (FTS5).
- **`facts`**: Responsible for the L1 fact extraction from OCR texts using rules (e.g., extracting phone numbers and amounts).
- **`brain`**: Interacts with the user-configured CLI agent for L2 assumptions and memory queries.
- **`consent`**: Manages the privacy consents necessary for recording, processing, and outbound data.

### `sister-capture`
Handles platform-specific screen capture, OCR, and accessibility (UIA, AX) context extraction.

- **Windows**: Uses Windows.Graphics.Capture or Desktop Duplication, Windows OCR, and UIA.
- **macOS**: Uses ScreenCaptureKit, Vision OCR, and Accessibility APIs.
- **Linux (X11)**: Uses X11 capture and Tesseract OCR.

### `sister-cli`
The command-line interface for interacting with the AI-Sister system. Includes commands to record, query, replay, and manage memory.

### `sister-shell`
Contains shared logic for the desktop shell, including hit-testing for click-through support and desktop UI utilities.

### `sister-tts`
Manages Text-to-Speech capabilities, handling the local speech playback and the optional Azure TTS fallback.

### `sister-assets`
Manages the bundled visual assets (e.g., character images) and their associated metadata and permissions.

### `sister-usage`
Handles usage limits and public status APIs for the desktop companion.
