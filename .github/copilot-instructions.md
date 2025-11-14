# GitHub Copilot Instructions for kube-incus

## Project Overview

kube-incus is a project that introduces `distrobuilder` system-containers into the kube-virt ecosystem. This project bridges the gap between Incus container technology and Kubernetes virtualization.

## Development Guidelines

### Project Context
- **Primary Technologies**: Incus containers, distrobuilder, kube-virt
- **Goal**: Integration of system containers into Kubernetes virtualization
- **Architecture**: Container-based virtualization within Kubernetes

### Coding Standards
- Write clear, maintainable code with appropriate comments
- Follow the existing code style and conventions in the repository
- Use meaningful variable and function names
- Keep functions focused and modular

### Testing
- Write tests for new functionality
- Ensure existing tests pass before submitting changes
- Test edge cases and error conditions
- Document test scenarios

### Documentation
- Update README.md when adding new features
- Document complex logic and architectural decisions
- Include usage examples for new functionality
- Keep inline comments up to date

### Dependencies
- Minimize external dependencies
- Document why new dependencies are needed
- Prefer well-maintained, widely-used libraries
- Check for security vulnerabilities before adding dependencies

### Version Control
- Write clear, descriptive commit messages
- Keep commits atomic and focused
- Reference issue numbers in commit messages when applicable
- Avoid committing generated files or build artifacts

### Best Practices for kube-incus
- Consider compatibility with both Incus and kube-virt ecosystems
- Follow Kubernetes best practices for container management
- Ensure security best practices for system containers
- Test integration points between components
- Document configuration options and environment variables

## Helpful Commands

When working with this repository, consider these common tasks:
- Build: [To be added based on project structure]
- Test: [To be added based on project structure]
- Deploy: [To be added based on project structure]

## Resources
- Incus Documentation: https://linuxcontainers.org/incus/
- KubeVirt Documentation: https://kubevirt.io/
- distrobuilder: https://github.com/lxc/distrobuilder
