# There I Was
[![Build thereiwas-backend](https://github.com/flying7eleven/thereiwas/actions/workflows/build_server.yml/badge.svg)](https://github.com/flying7eleven/thereiwas/actions/workflows/build_server.yml)

**There I Was** is an open-source API, written in Rust, that automatically receives and stores geocoordinates sent from a smartphone.
Inspired by services like Google Maps Timeline / Location History, this project aims to provide a self-hosted, privacy-focused way to track and visualize your location history over the years.

Ever wonder where you’ve been throughout the year?
**There I Was** helps you answer that question by recording your coordinates in a simple, flexible, and private manner, all without relying on a closed, proprietary service.
It’s a passion project that I find exciting — especially at the end of each year when I look back at my journey.

## Usage

### Send Location Data

Your smartphone or device should periodically send POST requests with JSON body to the corresponding API endpoint.
There are different endpoint implementations for different Apps which can submit the location.
Chose accordingly:

#### OwnTracks
If you want to use the [OwnTracks](https://owntracks.org/) App, configure the app to POST the location information to the endpoint `/api/v1/location/owntracks`.
For doing it manually, you can use the following example body which has to be POSTed to the endpoint mentioned above:

```json
{
    "_type": "location",
    "_id": "randomid",
    "bs": 0,
    "lat": 52.5200,
    "lon": 13.4050,
    "tst": 1735059600,
    "topic": "/devices/bob/phone"
}
```

The endpoint should answer with an `204 No Content` indicating that the data set was stored successfully.

### Retrieve Location Data
By sending a GET request to `/api/v1/locations` you will receive the stored coordinates (potentially paginated and filterable in the future).

### Visualization
Currently, the project only focuses on storing and retrieving data.
A frontend or a data visualization tool (e.g., Grafana, Kibana, or a custom app) can then consume the API to display your location history on maps or in charts.
A frontend will be developed later on.
