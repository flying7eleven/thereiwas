import { useEffect, useState } from "react";
import { MapContainer, TileLayer, Marker, Popup, useMap } from "react-leaflet";
import { Box, Paper, Typography } from "@mui/material";
import { LatLngExpression } from "leaflet";

interface Position {
  longitude: number;
  latitude: number;
  horizontal_accuracy: number;
  vertical_accuracy: number;
  altitude: number;
  measurement_time: number;
}

const MIN_HORIZONTAL_ACCURACY = 15; // minimum accuracy threshold in meters

const BoundsUpdater = ({ positions }: { positions: Position[] }) => {
  const map = useMap();

  useEffect(() => {
    if (positions.length > 0) {
      const bounds = positions.reduce(
        (bounds, position) =>
          bounds.extend([position.latitude, position.longitude]),
        map.getBounds(),
      );
      map.fitBounds(bounds, { padding: [50, 50] });
    }
  }, [positions, map]);

  return null;
};

export const DashboardView = () => {
  const [positions, setPositions] = useState<Position[]>([]);
  const [mapCenter, setMapCenter] = useState<LatLngExpression>([
    51.235344, 6.782973,
  ]);

  useEffect(() => {
    const fetchPositions = async () => {
      try {
        const response = await fetch("http://localhost:3000/v1/positions");
        if (!response.ok) {
          throw new Error("Failed to fetch positions");
        }
        const data: Position[] = await response.json();

        // Filter positions by horizontal accuracy
        const accuratePositions = data.filter(
          (position) => position.horizontal_accuracy <= MIN_HORIZONTAL_ACCURACY,
        );

        setPositions(accuratePositions);

        // Update map center to first accurate position if available
        if (accuratePositions.length > 0) {
          setMapCenter([
            accuratePositions[0].latitude,
            accuratePositions[0].longitude,
          ]);
        }
      } catch (error) {
        console.error("Error fetching positions:", error);
      }
    };

    fetchPositions();
    const interval = setInterval(fetchPositions, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Box
      sx={{
        width: "100%",
        height: "100vh",
        display: "flex",
        flexDirection: "column",
        p: 2,
        boxSizing: "border-box",
      }}
    >
      <Paper
        elevation={3}
        sx={{
          flex: 1,
          width: "100%",
          overflow: "hidden",
          position: "relative",
        }}
      >
        <MapContainer
          center={mapCenter}
          zoom={13}
          style={{
            height: "100%",
            width: "100%",
            position: "absolute",
            top: 0,
            left: 0,
          }}
          maxZoom={19}
          preferCanvas={true}
        >
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            maxZoom={19}
          />
          <BoundsUpdater positions={positions} />
          {positions.map((position, index) => (
            <Marker
              key={`${position.latitude}-${position.longitude}-${index}`}
              position={
                [position.latitude, position.longitude] as LatLngExpression
              }
            >
              <Popup>
                <Typography variant="body2">
                  Altitude: {position.altitude}m<br />
                  Time:{" "}
                  {new Date(position.measurement_time * 1000).toLocaleString()}
                  <br />
                  Accuracy: H:{position.horizontal_accuracy}m V:
                  {position.vertical_accuracy}m
                </Typography>
              </Popup>
            </Marker>
          ))}
        </MapContainer>
      </Paper>
    </Box>
  );
};
