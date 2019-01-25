# \GroupApi

All URIs are relative to *https://your-subdomain.okta.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_rule**](GroupApi.md#activate_rule) | **Post** /api/v1/groups/rules/{ruleId}/lifecycle/activate | Activate a group Rule
[**add_user_to_group**](GroupApi.md#add_user_to_group) | **Put** /api/v1/groups/{groupId}/users/{userId} | Add User to Group
[**create_group**](GroupApi.md#create_group) | **Post** /api/v1/groups | Add Group
[**create_rule**](GroupApi.md#create_rule) | **Post** /api/v1/groups/rules | Create Group Rule
[**deactivate_rule**](GroupApi.md#deactivate_rule) | **Post** /api/v1/groups/rules/{ruleId}/lifecycle/deactivate | Deactivate a group Rule
[**delete_group**](GroupApi.md#delete_group) | **Delete** /api/v1/groups/{groupId} | Remove Group
[**delete_rule**](GroupApi.md#delete_rule) | **Delete** /api/v1/groups/rules/{ruleId} | Delete a group Rule
[**get_group**](GroupApi.md#get_group) | **Get** /api/v1/groups/{groupId} | List Group Rules
[**get_rule**](GroupApi.md#get_rule) | **Get** /api/v1/groups/rules/{ruleId} | Get Group Rule
[**list_group_users**](GroupApi.md#list_group_users) | **Get** /api/v1/groups/{groupId}/users | List Group Members
[**list_groups**](GroupApi.md#list_groups) | **Get** /api/v1/groups | List Groups
[**list_rules**](GroupApi.md#list_rules) | **Get** /api/v1/groups/rules | List Group Rules
[**remove_group_user**](GroupApi.md#remove_group_user) | **Delete** /api/v1/groups/{groupId}/users/{userId} | Remove User from Group
[**update_group**](GroupApi.md#update_group) | **Put** /api/v1/groups/{groupId} | Update Group
[**update_rule**](GroupApi.md#update_rule) | **Put** /api/v1/groups/rules/{ruleId} | 


# **activate_rule**
> activate_rule(ctx, rule_id)
Activate a group Rule

Activates a specific group rule by id from your organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rule_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **add_user_to_group**
> add_user_to_group(ctx, group_id, user_id)
Add User to Group

Adds a [user](users.html#user-model) to a group with `OKTA_GROUP` type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 
  **user_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_group**
> crate::models::Group create_group(ctx, group)
Add Group

Adds a new group with `OKTA_GROUP` type to your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group** | [**Group**](Group.md)|  | 

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rule**
> crate::models::GroupRule create_rule(ctx, group_rule)
Create Group Rule

Creates a group rule to dynamically add users to the specified group if they match the condition

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_rule** | [**GroupRule**](GroupRule.md)|  | 

### Return type

[**crate::models::GroupRule**](GroupRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deactivate_rule**
> deactivate_rule(ctx, rule_id)
Deactivate a group Rule

Deactivates a specific group rule by id from your organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rule_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_group**
> delete_group(ctx, group_id)
Remove Group

Removes a group with `OKTA_GROUP` type from your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rule**
> delete_rule(ctx, rule_id, optional)
Delete a group Rule

Removes a specific group rule by id from your organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rule_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rule_id** | **String**|  | 
 **remove_users** | **bool**|  | [default to false]

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_group**
> crate::models::Group get_group(ctx, group_id, optional)
List Group Rules

Lists all group rules for your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_rule**
> crate::models::GroupRule get_rule(ctx, rule_id, optional)
Get Group Rule

Fetches a specific group rule by id from your organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rule_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **rule_id** | **String**|  | 
 **expand** | **String**|  | 

### Return type

[**crate::models::GroupRule**](GroupRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_group_users**
> Vec<crate::models::User> list_group_users(ctx, group_id, optional)
List Group Members

Enumerates all [users](/docs/api/resources/users.html#user-model) that are a member of a group.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **group_id** | **String**|  | 
 **after** | **String**| Specifies the pagination cursor for the next page of users | 
 **limit** | **i32**| Specifies the number of user results in a page | [default to -1]
 **managed_by** | **String**|  | [default to all]

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_groups**
> Vec<crate::models::Group> list_groups(ctx, optional)
List Groups

Enumerates groups in your organization with pagination. A subset of groups can be returned that match a supported filter expression or query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **q** | **String**| Searches the name property of groups for matching value | 
 **filter** | **String**| Filter expression for groups | 
 **after** | **String**| Specifies the pagination cursor for the next page of groups | 
 **limit** | **i32**| Specifies the number of group results in a page | [default to -1]
 **expand** | **String**|  | 

### Return type

[**Vec<crate::models::Group>**](Group.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rules**
> Vec<crate::models::GroupRule> list_rules(ctx, optional)
List Group Rules

Lists all group rules for your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Specifies the number of rule results in a page | [default to -1]
 **after** | **String**| Specifies the pagination cursor for the next page of rules | 
 **expand** | **String**|  | 

### Return type

[**Vec<crate::models::GroupRule>**](GroupRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_group_user**
> remove_group_user(ctx, group_id, user_id)
Remove User from Group

Removes a [user](users.html#user-model) from a group with `OKTA_GROUP` type.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 
  **user_id** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_group**
> crate::models::Group update_group(ctx, group_id, group)
Update Group

Updates the profile for a group with `OKTA_GROUP` type from your organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **group_id** | **String**|  | 
  **group** | [**Group**](Group.md)|  | 

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_rule**
> crate::models::GroupRule update_rule(ctx, rule_id, group_rule)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rule_id** | **String**|  | 
  **group_rule** | [**GroupRule**](GroupRule.md)|  | 

### Return type

[**crate::models::GroupRule**](GroupRule.md)

### Authorization

[api_token](../README.md#api_token)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

