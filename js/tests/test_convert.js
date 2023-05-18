import { Wkt } from '../proj4wkt.js';

var fixtures = [
    {
        name: "Test WKT1 NAD83 Projection",
        src: String.raw`PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83",`+
             String.raw`DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,` +
             String.raw`AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,` +
             String.raw`AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,` +
             String.raw`AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,` +
             String.raw`AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],` +
             String.raw`PARAMETER["standard_parallel_1",42.68333333333333],` +
             String.raw`PARAMETER["standard_parallel_2",41.71666666666667],` +
             String.raw`PARAMETER["latitude_of_origin", -41],PARAMETER["central_meridian",-71.5],` +
             String.raw`PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],` +
             String.raw`AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]`,
        expect: "+proj=lcc +lat_1=42.68333333333333 +lat_2=41.71666666666667" +
                " +lat_0=-41 +lon_0=-71.5 +x_0=200000 +y_0=750000 +units=m +a=6378137" +
                " +rf=298.257222101 +towgs84=0,0,0,0,0,0,0"
    }
];


fixtures.forEach(function(item) {
        let res = Wkt.to_proj(item.src);
        console.assert(res === item.expect);
        console.log(`${item.name}...Ok`);
});


