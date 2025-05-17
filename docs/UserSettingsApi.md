# \UserSettingsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_setting_by_id**](UserSettingsApi.md#delete_user_setting_by_id) | **DELETE** /user_settings/delete/{userSettingId} | Delete user setting by id
[**get_user_setting_by_id**](UserSettingsApi.md#get_user_setting_by_id) | **GET** /user_settings/view/{userSettingId} | Get user setting by id
[**get_user_setting_by_name**](UserSettingsApi.md#get_user_setting_by_name) | **GET** /user_settings/getSetting/{userId}/{userSettingName} | Get user setting by id
[**get_user_settings**](UserSettingsApi.md#get_user_settings) | **GET** /user_settings | Get user settings
[**search_user_settings**](UserSettingsApi.md#search_user_settings) | **POST** /user_settings | Search user settings
[**set_user_setting**](UserSettingsApi.md#set_user_setting) | **POST** /user_settings/setSetting/{userId}/{userSettingName} | Set user setting



## delete_user_setting_by_id

> models::DeleteUserSettingById200Response delete_user_setting_by_id(user_setting_id)
Delete user setting by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_setting_id** | **String** | UUID or numeric ID of the user setting | [required] |

### Return type

[**models::DeleteUserSettingById200Response**](deleteUserSettingById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_setting_by_id

> models::GetUserSettings200ResponseInner get_user_setting_by_id(user_setting_id)
Get user setting by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_setting_id** | **String** | UUID or numeric ID of the user setting | [required] |

### Return type

[**models::GetUserSettings200ResponseInner**](getUserSettings_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_setting_by_name

> models::GetUserSettings200ResponseInner get_user_setting_by_name(user_id, user_setting_name)
Get user setting by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |
**user_setting_name** | [**UserSettingName**](.md) | Name of the user setting | [required] |

### Return type

[**models::GetUserSettings200ResponseInner**](getUserSettings_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_settings

> Vec<models::GetUserSettings200ResponseInner> get_user_settings()
Get user settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetUserSettings200ResponseInner>**](getUserSettings_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_user_settings

> Vec<models::GetUserSettings200ResponseInner> search_user_settings(search_user_settings_request)
Search user settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_user_settings_request** | Option<[**SearchUserSettingsRequest**](SearchUserSettingsRequest.md)> |  |  |

### Return type

[**Vec<models::GetUserSettings200ResponseInner>**](getUserSettings_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_setting

> models::GetUserSettings200ResponseInner set_user_setting(user_id, user_setting_name, set_user_setting_request)
Set user setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Numeric ID of the user | [required] |
**user_setting_name** | [**UserSettingName**](.md) | Name of the user setting | [required] |
**set_user_setting_request** | Option<[**SetUserSettingRequest**](SetUserSettingRequest.md)> |  |  |

### Return type

[**models::GetUserSettings200ResponseInner**](getUserSettings_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

