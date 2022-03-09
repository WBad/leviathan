/*
# Generate authentication strings (REST API)

## Inputs

### PostData
postData is a "&" concatenation in the form <argument>=<value> and is specific to each REST endpoint.

Example
To operate the endpoint orderbook you choose the argument symbol with value
fi_xbtusd_180615.
postData is then given by symbol=fi_xbtusd_180615.


### Nonce
nonce is a continuously incrementing integer parameter.
A good nonce is your system time in milliseconds (in string format).
Our system tolerates nonces that are out of order for a brief period of time. Nonce is not required.

Example
1415957147987

Many authentication issues are related with incorrect nonce. A new pair of API keys will automatically reset the nonce and resolve these issues.


### Endpoint Path
endpointPath This is the URL extension of the endpoint.

Example
/api/v3/orderbook


### API Secret
The api_secret is obtained as described in the previous section.

Example
rttp4AzwRfYEdQ7R7X8Z/04Y4TZPa97pqCypi3xXxAqftygftnI6H9yGV+O
cUOOJeFtZkr8mVwbAndU3Kz4Q+eG


## Generate Authent

Based on these inputs, authent needs to be computed as follows:
    1. Concatenate postData + nonce + endpointPath
    2. Hash the result of step 1 with the SHA-256 algorithm
    3. Base64-decode your api_secret
    4. Use the result of step 3 to hash the result of the step 2 with the HMAC-SHA-512 algorithm
    5. Base64-encode the result of step 4
 */
pub fn get_authent(post_data: &str, nonce: &str, endpoint_path: &str, api_secret: &str) {
    todo!()
}
