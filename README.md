# There I Was
[![Analyze: Frontend](https://github.com/flying7eleven/thereiwas/actions/workflows/analyze_frontend.yml/badge.svg)](https://github.com/flying7eleven/thereiwas/actions/workflows/analyze_frontend.yml)
[![Analyze: Backend](https://github.com/flying7eleven/thereiwas/actions/workflows/analyze_backend.yml/badge.svg)](https://github.com/flying7eleven/thereiwas/actions/workflows/analyze_backend.yml)
[![Build: Frontend](https://github.com/flying7eleven/thereiwas/actions/workflows/build_frontend.yml/badge.svg)](https://github.com/flying7eleven/thereiwas/actions/workflows/build_frontend.yml)
[![Build: Backend](https://github.com/flying7eleven/thereiwas/actions/workflows/build_backend.yml/badge.svg)](https://github.com/flying7eleven/thereiwas/actions/workflows/build_backend.yml)

**There I Was** is an open-source API, written in Rust, that automatically receives and stores geocoordinates sent from a smartphone.
Inspired by services like Google Maps Timeline / Location History, this project aims to provide a self-hosted, privacy-focused way to track and visualize your location history over the years.

Ever wonder where you’ve been throughout the year?
**There I Was** helps you answer that question by recording your coordinates in a simple, flexible, and private manner, all without relying on a closed, proprietary service.
It’s a passion project that I find exciting — especially at the end of each year when I look back at my journey.

## Development

### Backend

```shell
cargo upgrade -i allow && cargo update
```

```shell
git log backend-2025.3.4..HEAD --oneline
```
