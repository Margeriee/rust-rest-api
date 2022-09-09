# rust-rest-api

Create REST API with following endpoints:

**GET: /calculateDisselUsageForDistance**
The endpoint should return a number, which is the fuel
consumption on specified distance.

Inputs in request url(query parameters):
-> distance- total distance between point A and point B.
Provided as a natural number. Please assume that unit
measurement here is KM.
-> yearOfProduction - year of production of the car.
Provided as a number.
-> fuelUsagePer100KM - natural number that represents
approximate fuel consumption per 100KM. Provided as
a number.

Returns:
-> fuelUsage- based on input, please perform calculations
that will allow to define fuelConsumotion.

**GET: /probabilityOfUnitInjectorFail**
The endpoint should return a percentage of the chance that the
engine of the C6 model will fail on the Unit Injector element.
Meaning “0” means there is no such possibility, and “0,77” means
that there is a 77% chance that the Unit Injector will fail.

Inputs in request url(query parameters):
-> VIN - not relevant, but customer really wants it here

Returns:
-> failProbability - beforehand there were extensive R&D
tasks performed, AI was used, we even searched
through Google, including third and fourth page. All
effort in the name of finding a way to calculate such a
chance. Results show that randomly generated
percentages that do not base on anything, accurately
represent the chances of potential failure. Please use
some method to generate random numbers from 0 to
100 and convert to the correct format.