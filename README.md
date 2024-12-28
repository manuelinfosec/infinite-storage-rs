# Data Compression and Video-Based File Storage System

## Overview
This project explores innovative approaches to data storage through video compression techniques, specifically focusing on the encoding of arbitrary files into video formats. The implementation is written in Rust, chosen for its memory safety and performance characteristics.

## Project Objectives
- Demonstrate novel approaches to data compression and storage
- Explore the limitations and possibilities of video-based file storage
- Implement efficient encoding and decoding algorithms
- Study the effects of video compression on data integrity

## Technical Implementation

### Core Technologies
- **Primary Language**: Rust
- **Key Dependencies**:
  - OpenCV for video processing
  - youtube-dl for downloading media
  - Clang for compilation support

### System Architecture

The system implements two distinct encoding modes:

#### RGB Mode
- Utilizes RGB pixel values for data encoding
- Each pixel stores 3 bytes of data
- Offers higher storage efficiency
- Sensitive to compression artifacts

#### Binary Mode
- Implements binary encoding using black and white pixels
- Enhanced compression resistance
- Uses 2x2 pixel blocks for improved stability
- Lower storage efficiency but higher reliability

### Installation

#### Method 1: Building from Source
Prerequisites:
- Rust toolchain
- OpenCV libraries
- FFmpeg
- Clang
  
```bash
git clone https://github.com/manuelinfosec/infinite-storage-rs
cargo build --release
```

#### Method 2: Docker Deployment
```bash
# Build Docker image
docker build -t data-storage-system .

# Build project
docker run -it --rm -v ${PWD}:/home/project data-storage-system cargo build --release
```

### Usage Instructions

1. **File Preparation**
   - Compress target files into a ZIP archive
   - Launch the application
   - Select encoding parameters

2. **Encoding Process**
   - Choose encoding mode (RGB/Binary)
   - Process file for video conversion
   - Store resulting video file

3. **Decoding Process**
   - Input encoded video
   - Extract original data
   - Verify file integrity

### Technical Specifications

The system includes several key features:
- Automated encoding parameter detection
- First-frame metadata storage
- Compression resistance optimization
- Error detection and handling

### Performance Considerations
- File size expansion ratio: approximately 4:1 in optimal settings
- RAM usage limitations: ~100MB maximum file size
- Processing speed varies based on encoding mode

<!-- ### Limitations and Future Work
- Current implementation focuses on proof-of-concept
- Memory optimization opportunities exist
- Potential for filesystem integration
- Scope for improved compression algorithms

## Academic Context
This project demonstrates practical applications of:
- Data compression algorithms
- Video processing techniques
- Error correction methodologies
- Systems programming in Rust

## Technical Documentation
For detailed technical documentation, including API references and implementation details, please refer to the `/docs` directory.

## Acknowledgments
This project draws inspiration from various academic works in the field of data storage and compression techniques. Special thanks to:
- [List relevant academic references]
- [Any mentors or advisors] -->
