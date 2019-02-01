# \PolicyApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_policy**](PolicyApi.md#activate_policy) | **post** /api/v1/policies/{policyId}/lifecycle/activate | 
[**activate_policy_rule**](PolicyApi.md#activate_policy_rule) | **post** /api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/activate | 
[**add_policy_rule**](PolicyApi.md#add_policy_rule) | **post** /api/v1/policies/{policyId}/rules | 
[**create_policy**](PolicyApi.md#create_policy) | **post** /api/v1/policies | 
[**deactivate_policy**](PolicyApi.md#deactivate_policy) | **post** /api/v1/policies/{policyId}/lifecycle/deactivate | 
[**deactivate_policy_rule**](PolicyApi.md#deactivate_policy_rule) | **post** /api/v1/policies/{policyId}/rules/{ruleId}/lifecycle/deactivate | 
[**delete_policy**](PolicyApi.md#delete_policy) | **delete** /api/v1/policies/{policyId} | 
[**delete_policy_rule**](PolicyApi.md#delete_policy_rule) | **delete** /api/v1/policies/{policyId}/rules/{ruleId} | 
[**get_policy**](PolicyApi.md#get_policy) | **get** /api/v1/policies/{policyId} | 
[**get_policy_rule**](PolicyApi.md#get_policy_rule) | **get** /api/v1/policies/{policyId}/rules/{ruleId} | 
[**list_policies**](PolicyApi.md#list_policies) | **get** /api/v1/policies | 
[**list_policy_rules**](PolicyApi.md#list_policy_rules) | **get** /api/v1/policies/{policyId}/rules | 
[**update_policy**](PolicyApi.md#update_policy) | **put** /api/v1/policies/{policyId} | 
[**update_policy_rule**](PolicyApi.md#update_policy_rule) | **put** /api/v1/policies/{policyId}/rules/{ruleId} | 


# **activate_policy**
> activate_policy(ctx, policy_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **activate_policy_rule**
> activate_policy_rule(ctx, policy_id, rule_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **rule_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **add_policy_rule**
> ::models::PolicyRule add_policy_rule(ctx, policy_id, policy_rule, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **policy_rule** | [**PolicyRule**](PolicyRule.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **policy_id** | **String**|  | 
 **policy_rule** | [**PolicyRule**](PolicyRule.md)|  | 
 **activate** | **bool**|  | [default to true]

### Return type

[**::models::PolicyRule**](PolicyRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_policy**
> ::models::Policy create_policy(ctx, policy, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy** | [**Policy**](Policy.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **policy** | [**Policy**](Policy.md)|  | 
 **activate** | **bool**|  | [default to true]

### Return type

[**::models::Policy**](Policy.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deactivate_policy**
> deactivate_policy(ctx, policy_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deactivate_policy_rule**
> deactivate_policy_rule(ctx, policy_id, rule_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **rule_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_policy**
> delete_policy(ctx, policy_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_policy_rule**
> delete_policy_rule(ctx, policy_id, rule_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **rule_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_policy**
> ::models::Policy get_policy(ctx, policy_id, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **policy_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**::models::Policy**](Policy.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_policy_rule**
> ::models::PolicyRule get_policy_rule(ctx, policy_id, rule_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **rule_id** | **String**|  | 

### Return type

[**::models::PolicyRule**](PolicyRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_policies**
> Vec<::models::Policy> list_policies(ctx, _type, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **_type** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**|  | 
 **status** | **String**|  | 
 **after** | **String**|  | 
 **limit** | **i32**|  | [default to -1]
 **expand** | **String**|  | 

### Return type

[**Vec<::models::Policy>**](Policy.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_policy_rules**
> Vec<::models::PolicyRule> list_policy_rules(ctx, policy_id)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 

### Return type

[**Vec<::models::PolicyRule>**](PolicyRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_policy**
> ::models::Policy update_policy(ctx, policy_id, policy)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **policy** | [**Policy**](Policy.md)|  | 

### Return type

[**::models::Policy**](Policy.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_policy_rule**
> ::models::PolicyRule update_policy_rule(ctx, policy_id, rule_id, policy_rule)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **policy_id** | **String**|  | 
  **rule_id** | **String**|  | 
  **policy_rule** | [**PolicyRule**](PolicyRule.md)|  | 

### Return type

[**::models::PolicyRule**](PolicyRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

