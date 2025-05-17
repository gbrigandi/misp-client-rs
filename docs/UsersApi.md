# \UsersApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_user**](UsersApi.md#add_user) | **POST** /admin/users/add | Add user
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /admin/users/delete/{userId} | Delete user
[**delete_user_totp**](UsersApi.md#delete_user_totp) | **DELETE** /users/totp_delete/{userId} | Delete user TOTP
[**edit_user**](UsersApi.md#edit_user) | **PUT** /admin/users/edit/{userId} | Edit user
[**get_user_by_id**](UsersApi.md#get_user_by_id) | **GET** /admin/users/view/{userId} | Get user by ID
[**get_users**](UsersApi.md#get_users) | **GET** /admin/users | Get users
[**reset_user_password**](UsersApi.md#reset_user_password) | **POST** /users/initiatePasswordReset/{userId}/{firstTimeReset} | Reset user password



## add_user

> models::User add_user(user_no_id)
Add user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_no_id** | Option<[**UserNoId**](UserNoId.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> models::DeleteUser200Response delete_user(user_id)
Delete user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |

### Return type

[**models::DeleteUser200Response**](deleteUser_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_totp

> models::DeleteUserTotp200Response delete_user_totp(user_id)
Delete user TOTP

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |

### Return type

[**models::DeleteUserTotp200Response**](deleteUserTotp_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_user

> models::User edit_user(user_id, user)
Edit user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |
**user** | Option<[**User**](User.md)> |  |  |

### Return type

[**models::User**](User.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_by_id

> models::ExtendedUser get_user_by_id(user_id)
Get user by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |

### Return type

[**models::ExtendedUser**](ExtendedUser.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users

> Vec<models::UserListItem> get_users()
Get users

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::UserListItem>**](UserListItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_password

> models::ResetUserPassword200Response reset_user_password(user_id, first_time_reset)
Reset user password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |
**first_time_reset** | **String** | First time reset is set to `1` only  for new user registrations. | [required] |

### Return type

[**models::ResetUserPassword200Response**](resetUserPassword_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

