export interface ClusterResponse {
  x: number;
  y: number;
  weight: number;
}

export interface WeightedClusterCenter {
  x: number;
  y: number;
  center: {
    lat: number;
    lng: number;
  };
  weight: number;
  normalisedWeight: number;
}
