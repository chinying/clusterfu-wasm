export interface ClusterResponse {
  x: number;
  y: number;
  weight: number;
}

export interface WeightedClusterCenter {
  center: {
    lat: number;
    lng: number;
  };
  weight: number;
}
