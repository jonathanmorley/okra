# \LogApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_logs**](LogApi.md#get_logs) | **get** /api/v1/logs | Fetch a list of events from your Okta organization system log.


# **get_logs**
> Vec<::models::LogEvent> get_logs(ctx, optional)
Fetch a list of events from your Okta organization system log.

The Okta System Log API provides read access to your organization’s system log. This API provides more functionality than the Events API

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **until** | **String**|  | 
 **since** | **String**|  | 
 **filter** | **String**|  | 
 **q** | **String**|  | 
 **limit** | **i32**|  | [default to 100]
 **sort_order** | **String**|  | [default to ASCENDING]
 **after** | **String**|  | 

### Return type

[**Vec<::models::LogEvent>**](LogEvent.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

