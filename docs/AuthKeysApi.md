# \AuthKeysApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_auth_key**](AuthKeysApi.md#add_auth_key) | **POST** /auth_keys/add/{userId} | Add auth keys
[**delete_auth_key**](AuthKeysApi.md#delete_auth_key) | **DELETE** /auth_keys/delete/{authKeyId} | Delete auth key
[**edit_auth_key**](AuthKeysApi.md#edit_auth_key) | **POST** /auth_keys/edit/{authKeyId} | Edit auth key
[**get_auth_key_by_id**](AuthKeysApi.md#get_auth_key_by_id) | **GET** /auth_keys/view/{authKeyId} | View auth key
[**get_auth_keys**](AuthKeysApi.md#get_auth_keys) | **GET** /auth_keys | Get auth keys
[**search_auth_keys**](AuthKeysApi.md#search_auth_keys) | **POST** /auth_keys | Search auth keys



## add_auth_key

> models::AddAuthKey200Response add_auth_key(user_id, add_auth_key_request)
Add auth keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |
**add_auth_key_request** | Option<[**AddAuthKeyRequest**](AddAuthKeyRequest.md)> |  |  |

### Return type

[**models::AddAuthKey200Response**](addAuthKey_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_auth_key

> models::DeleteAuthKey200Response delete_auth_key(auth_key_id)
Delete auth key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_key_id** | **String** | UUID or numeric ID of the auth key | [required] |

### Return type

[**models::DeleteAuthKey200Response**](deleteAuthKey_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_auth_key

> models::GetAuthKeyById200Response edit_auth_key(auth_key_id, edit_auth_key_request)
Edit auth key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_key_id** | **String** | UUID or numeric ID of the auth key | [required] |
**edit_auth_key_request** | Option<[**EditAuthKeyRequest**](EditAuthKeyRequest.md)> |  |  |

### Return type

[**models::GetAuthKeyById200Response**](getAuthKeyById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_key_by_id

> models::GetAuthKeyById200Response get_auth_key_by_id(auth_key_id)
View auth key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_key_id** | **String** | UUID or numeric ID of the auth key | [required] |

### Return type

[**models::GetAuthKeyById200Response**](getAuthKeyById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_keys

> Vec<models::GetAuthKeys200ResponseInner> get_auth_keys()
Get auth keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetAuthKeys200ResponseInner>**](getAuthKeys_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_auth_keys

> Vec<models::GetAuthKeys200ResponseInner> search_auth_keys(search_auth_keys_request)
Search auth keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_auth_keys_request** | Option<[**SearchAuthKeysRequest**](SearchAuthKeysRequest.md)> |  |  |

### Return type

[**Vec<models::GetAuthKeys200ResponseInner>**](getAuthKeys_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

