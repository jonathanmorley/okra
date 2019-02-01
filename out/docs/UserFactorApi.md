# \UserFactorApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_factor**](UserFactorApi.md#activate_factor) | **post** /api/v1/users/{userId}/factors/{factorId}/lifecycle/activate | Activate Factor
[**add_factor**](UserFactorApi.md#add_factor) | **post** /api/v1/users/{userId}/factors | Enroll Factor
[**delete_factor**](UserFactorApi.md#delete_factor) | **delete** /api/v1/users/{userId}/factors/{factorId} | 
[**get_factor**](UserFactorApi.md#get_factor) | **get** /api/v1/users/{userId}/factors/{factorId} | 
[**list_factors**](UserFactorApi.md#list_factors) | **get** /api/v1/users/{userId}/factors | 
[**list_supported_factors**](UserFactorApi.md#list_supported_factors) | **get** /api/v1/users/{userId}/factors/catalog | 
[**list_supported_security_questions**](UserFactorApi.md#list_supported_security_questions) | **get** /api/v1/users/{userId}/factors/questions | 
[**verify_factor**](UserFactorApi.md#verify_factor) | **post** /api/v1/users/{userId}/factors/{factorId}/verify | Verify MFA Factor


# **activate_factor**
> ::models::Factor activate_factor(ctx, user_id, factor_id, body)
Activate Factor

The `sms` and `token:software:totp` [factor types](#factor-type) require activation to complete the enrollment process.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 
  **factor_id** | **String**|  | 
  **body** | [**VerifyFactorRequest**](VerifyFactorRequest.md)|  | 

### Return type

[**::models::Factor**](Factor.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **add_factor**
> ::models::Factor add_factor(ctx, user_id, body, optional)
Enroll Factor

Enrolls a user with a supported [factor](#list-factors-to-enroll)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 
  **body** | [**Factor**](Factor.md)| Factor | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **String**|  | 
 **body** | [**Factor**](Factor.md)| Factor | 
 **update_phone** | **bool**|  | [default to false]
 **template_id** | **String**| id of SMS template (only for SMS factor) | 
 **token_lifetime_seconds** | **i32**|  | [default to 300]
 **activate** | **bool**|  | [default to false]

### Return type

[**::models::Factor**](Factor.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_factor**
> delete_factor(ctx, user_id, factor_id)


Unenrolls an existing factor for the specified user, allowing the user to enroll a new factor.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 
  **factor_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_factor**
> ::models::Factor get_factor(ctx, user_id, factor_id)


Fetches a factor for the specified user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 
  **factor_id** | **String**|  | 

### Return type

[**::models::Factor**](Factor.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_factors**
> Vec<::models::Factor> list_factors(ctx, user_id)


Enumerates all the enrolled factors for the specified user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 

### Return type

[**Vec<::models::Factor>**](Factor.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_supported_factors**
> Vec<::models::Factor> list_supported_factors(ctx, user_id)


Enumerates all the [supported factors](#supported-factors-for-providers) that can be enrolled for the specified user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 

### Return type

[**Vec<::models::Factor>**](Factor.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_supported_security_questions**
> Vec<::models::SecurityQuestion> list_supported_security_questions(ctx, user_id)


Enumerates all available security questions for a user's `question` factor

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 

### Return type

[**Vec<::models::SecurityQuestion>**](SecurityQuestion.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verify_factor**
> ::models::VerifyFactorResponse verify_factor(ctx, user_id, factor_id, body, optional)
Verify MFA Factor

Verifies an OTP for a `token` or `token:hardware` factor

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**|  | 
  **factor_id** | **String**|  | 
  **body** | [**VerifyFactorRequest**](VerifyFactorRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **String**|  | 
 **factor_id** | **String**|  | 
 **body** | [**VerifyFactorRequest**](VerifyFactorRequest.md)|  | 
 **template_id** | **String**|  | 
 **token_lifetime_seconds** | **i32**|  | [default to 300]

### Return type

[**::models::VerifyFactorResponse**](VerifyFactorResponse.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

