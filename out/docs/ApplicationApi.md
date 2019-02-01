# \ApplicationApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_application**](ApplicationApi.md#activate_application) | **post** /api/v1/apps/{appId}/lifecycle/activate | Activate Application
[**assign_user_to_application**](ApplicationApi.md#assign_user_to_application) | **post** /api/v1/apps/{appId}/users | Assign User to Application for SSO & Provisioning
[**clone_application_key**](ApplicationApi.md#clone_application_key) | **post** /api/v1/apps/{appId}/credentials/keys/{keyId}/clone | Clone Application Key Credential
[**create_application**](ApplicationApi.md#create_application) | **post** /api/v1/apps | Add Application
[**create_application_group_assignment**](ApplicationApi.md#create_application_group_assignment) | **put** /api/v1/apps/{appId}/groups/{groupId} | Assign Group to Application
[**deactivate_application**](ApplicationApi.md#deactivate_application) | **post** /api/v1/apps/{appId}/lifecycle/deactivate | Deactivate Application
[**delete_application**](ApplicationApi.md#delete_application) | **delete** /api/v1/apps/{appId} | Delete Application
[**delete_application_group_assignment**](ApplicationApi.md#delete_application_group_assignment) | **delete** /api/v1/apps/{appId}/groups/{groupId} | Remove Group from Application
[**delete_application_user**](ApplicationApi.md#delete_application_user) | **delete** /api/v1/apps/{appId}/users/{userId} | Remove User from Application
[**get_application**](ApplicationApi.md#get_application) | **get** /api/v1/apps/{appId} | Get Application
[**get_application_group_assignment**](ApplicationApi.md#get_application_group_assignment) | **get** /api/v1/apps/{appId}/groups/{groupId} | Get Assigned Group for Application
[**get_application_key**](ApplicationApi.md#get_application_key) | **get** /api/v1/apps/{appId}/credentials/keys/{keyId} | Get Key Credential for Application
[**get_application_user**](ApplicationApi.md#get_application_user) | **get** /api/v1/apps/{appId}/users/{userId} | Get Assigned User for Application
[**list_application_group_assignments**](ApplicationApi.md#list_application_group_assignments) | **get** /api/v1/apps/{appId}/groups | List Groups Assigned to Application
[**list_application_keys**](ApplicationApi.md#list_application_keys) | **get** /api/v1/apps/{appId}/credentials/keys | List Key Credentials for Application
[**list_application_users**](ApplicationApi.md#list_application_users) | **get** /api/v1/apps/{appId}/users | List Users Assigned to Application
[**list_applications**](ApplicationApi.md#list_applications) | **get** /api/v1/apps | List Applications
[**update_application**](ApplicationApi.md#update_application) | **put** /api/v1/apps/{appId} | Update Application
[**update_application_user**](ApplicationApi.md#update_application_user) | **post** /api/v1/apps/{appId}/users/{userId} | Update Application Profile for Assigned User


# **activate_application**
> activate_application(ctx, app_id)
Activate Application

Activates an inactive application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **assign_user_to_application**
> ::models::AppUser assign_user_to_application(ctx, app_id, app_user)
Assign User to Application for SSO & Provisioning

Assigns an user to an application with [credentials](#application-user-credentials-object) and an app-specific [profile](#application-user-profile-object). Profile mappings defined for the application are first applied before applying any profile properties specified in the request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **app_user** | [**AppUser**](AppUser.md)|  | 

### Return type

[**::models::AppUser**](AppUser.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **clone_application_key**
> ::models::JsonWebKey clone_application_key(ctx, app_id, key_id, target_aid)
Clone Application Key Credential

Clones a X.509 certificate for an application key credential from a source application to target application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **key_id** | **String**|  | 
  **target_aid** | **String**| Unique key of the target Application | 

### Return type

[**::models::JsonWebKey**](JsonWebKey.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_application**
> ::models::Application create_application(ctx, application, optional)
Add Application

Adds a new application to your Okta organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **application** | [**Application**](Application.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **application** | [**Application**](Application.md)|  | 
 **activate** | **bool**| Executes activation lifecycle operation when creating the app | [default to true]

### Return type

[**::models::Application**](Application.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_application_group_assignment**
> ::models::ApplicationGroupAssignment create_application_group_assignment(ctx, app_id, group_id, application_group_assignment)
Assign Group to Application

Assigns a group to an application

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **group_id** | **String**|  | 
  **application_group_assignment** | [**ApplicationGroupAssignment**](ApplicationGroupAssignment.md)|  | 

### Return type

[**::models::ApplicationGroupAssignment**](ApplicationGroupAssignment.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deactivate_application**
> deactivate_application(ctx, app_id)
Deactivate Application

Deactivates an active application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_application**
> delete_application(ctx, app_id)
Delete Application

Removes an inactive application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_application_group_assignment**
> delete_application_group_assignment(ctx, app_id, group_id)
Remove Group from Application

Removes a group assignment from an application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **group_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_application_user**
> delete_application_user(ctx, app_id, user_id, optional)
Remove User from Application

Removes an assignment for a user from an application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **user_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **user_id** | **String**|  | 
 **send_email** | **bool**|  | [default to false]

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application**
> ::models::Application get_application(ctx, app_id, optional)
Get Application

Fetches an application from your Okta organization by `id`.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**::models::Application**](Application.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_group_assignment**
> ::models::ApplicationGroupAssignment get_application_group_assignment(ctx, app_id, group_id, optional)
Get Assigned Group for Application

Fetches an application group assignment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **group_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **group_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**::models::ApplicationGroupAssignment**](ApplicationGroupAssignment.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_key**
> ::models::JsonWebKey get_application_key(ctx, app_id, key_id)
Get Key Credential for Application

Gets a specific [application key credential](#application-key-credential-model) by `kid`

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **key_id** | **String**|  | 

### Return type

[**::models::JsonWebKey**](JsonWebKey.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_application_user**
> ::models::AppUser get_application_user(ctx, app_id, user_id, optional)
Get Assigned User for Application

Fetches a specific user assignment for application by `id`.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **user_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **user_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**::models::AppUser**](AppUser.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_application_group_assignments**
> Vec<::models::ApplicationGroupAssignment> list_application_group_assignments(ctx, app_id, optional)
List Groups Assigned to Application

Enumerates group assignments for an application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **q** | **String**|  | 
 **after** | **String**| Specifies the pagination cursor for the next page of assignments | 
 **limit** | **i32**| Specifies the number of results for a page | [default to -1]
 **expand** | **String**|  | 

### Return type

[**Vec<::models::ApplicationGroupAssignment>**](ApplicationGroupAssignment.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_application_keys**
> Vec<::models::JsonWebKey> list_application_keys(ctx, app_id)
List Key Credentials for Application

Enumerates key credentials for an application

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 

### Return type

[**Vec<::models::JsonWebKey>**](JsonWebKey.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_application_users**
> Vec<::models::AppUser> list_application_users(ctx, app_id, optional)
List Users Assigned to Application

Enumerates all assigned [application users](#application-user-model) for an application.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **app_id** | **String**|  | 
 **q** | **String**|  | 
 **query_scope** | **String**|  | 
 **after** | **String**| specifies the pagination cursor for the next page of assignments | 
 **limit** | **i32**| specifies the number of results for a page | [default to -1]
 **filter** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**Vec<::models::AppUser>**](AppUser.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_applications**
> Vec<::models::Application> list_applications(ctx, optional)
List Applications

Enumerates apps added to your organization with pagination. A subset of apps can be returned that match a supported filter expression or query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **String**|  | 
 **after** | **String**| Specifies the pagination cursor for the next page of apps | 
 **limit** | **i32**| Specifies the number of results for a page | [default to -1]
 **filter** | **String**| Filters apps by status, user.id, group.id or credentials.signing.kid expression | 
 **expand** | **String**| Traverses users link relationship and optionally embeds Application User resource | 
 **include_non_deleted** | **bool**|  | [default to false]

### Return type

[**Vec<::models::Application>**](Application.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_application**
> ::models::Application update_application(ctx, app_id, application)
Update Application

Updates an application in your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **application** | [**Application**](Application.md)|  | 

### Return type

[**::models::Application**](Application.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_application_user**
> ::models::AppUser update_application_user(ctx, app_id, user_id, app_user)
Update Application Profile for Assigned User

Updates a user's profile for an application

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **app_id** | **String**|  | 
  **user_id** | **String**|  | 
  **app_user** | [**AppUser**](AppUser.md)|  | 

### Return type

[**::models::AppUser**](AppUser.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

