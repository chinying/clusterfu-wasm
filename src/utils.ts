import proj4 from "proj4";

proj4.defs([
  [
    "epsg:3414",
    "+proj=tmerc +lat_0=1.366666666666667 +lon_0=103.8333333333333 +k=1 +x_0=28001.642 +y_0=38744.572 +ellps=WGS84 +units=m +no_defs "
  ]
]);

const toWGS = proj4("epsg:3414").inverse;

export function XYToLatLng(
  x: number,
  y: number
): {
  lng: number;
  lat: number;
} {
  const latlng = toWGS([x, y]);
  return {
    lng: latlng[0],
    lat: latlng[1]
  };
}

export function mapToArray<T>(m: { [key: string]: T }): Array<T> {
  return Object.values(m);
}
