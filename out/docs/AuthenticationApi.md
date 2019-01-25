# \AuthenticationApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_activate_factor**](AuthenticationApi.md#auth_activate_factor) | **Post** /api/v1/authn/factors/{factorId}/lifecycle/activate | 
[**auth_change_password**](AuthenticationApi.md#auth_change_password) | **Post** /api/v1/authn/credentials/change_password | 
[**auth_verify_factor**](AuthenticationApi.md#auth_verify_factor) | **Post** /api/v1/authn/factors/{factorId}/verify | 
[**authenticate**](AuthenticationApi.md#authenticate) | **Post** /api/v1/authn | 
[**enroll_factor**](AuthenticationApi.md#enroll_factor) | **Post** /api/v1/authn/factors | 


# **auth_activate_factor**
> crate::models::AuthenticationTransaction auth_activate_factor(ctx, factor_id, body)


You can enroll, activate, manage, and verify factors inside the authentication context with /api/v1/authn/factors. This operation is only available for users that have not previously enrolled a factor and have transitioned to the MFA_ENROLL state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **factor_id** | **String**|  | 
  **body** | [**ActivateFactorRequest**](ActivateFactorRequest.md)|  | 

### Return type

[**crate::models::AuthenticationTransaction**](AuthenticationTransaction.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_change_password**
> crate::models::AuthenticationTransaction auth_change_password(ctx, body)


This operation changes a user’s password by providing the existing password and the new password password for authentication transactions with either the PASSWORD_EXPIRED or PASSWORD_WARN state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**ChangePasswordRequest**](ChangePasswordRequest.md)|  | 

### Return type

[**crate::models::AuthenticationTransaction**](AuthenticationTransaction.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **auth_verify_factor**
> crate::models::AuthenticationTransaction auth_verify_factor(ctx, factor_id, body, optional)


You can enroll, activate, manage, and verify factors inside the authentication context with /api/v1/authn/factors. This operation is only available for users that have not previously enrolled a factor and have transitioned to the MFA_ENROLL state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **factor_id** | **String**|  | 
  **body** | [**AuthVerifyFactorRequest**](AuthVerifyFactorRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **factor_id** | **String**|  | 
 **body** | [**AuthVerifyFactorRequest**](AuthVerifyFactorRequest.md)|  | 
 **remember_device** | **bool**| user’s decision to remember device | [default to false]
 **auto_push** | **bool**| user’s decision to send push to device automatically | [default to false]

### Return type

[**crate::models::AuthenticationTransaction**](AuthenticationTransaction.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authenticate**
> crate::models::AuthenticationTransaction authenticate(ctx, body)


Every authentication transaction starts with primary authentication which validates a user’s primary password credential. Password Policy, MFA Policy, and Sign-On Policy are evaluated during primary authentication to determine if the user’s password is expired, a factor should be enrolled, or additional verification is required. The transaction state of the response depends on the user’s status, group memberships and assigned policies.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**AuthenticationRequest**](AuthenticationRequest.md)|  | 

### Return type

[**crate::models::AuthenticationTransaction**](AuthenticationTransaction.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **enroll_factor**
> crate::models::AuthenticationTransaction enroll_factor(ctx, body)


You can enroll, activate, manage, and verify factors inside the authentication context with /api/v1/authn/factors. This operation is only available for users that have not previously enrolled a factor and have transitioned to the MFA_ENROLL state.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**EnrollFactorRequest**](EnrollFactorRequest.md)|  | 

### Return type

[**crate::models::AuthenticationTransaction**](AuthenticationTransaction.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

