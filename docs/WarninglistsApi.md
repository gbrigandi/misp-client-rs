# \WarninglistsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_value_warninglists_matches**](WarninglistsApi.md#check_value_warninglists_matches) | **POST** /warninglists/checkValue | Check if a list of values matches any warninglists
[**get_warninglist_by_id**](WarninglistsApi.md#get_warninglist_by_id) | **GET** /warninglists/view/{warninglistId} | Get warninglist by ID
[**get_warninglists**](WarninglistsApi.md#get_warninglists) | **GET** /warninglists | Get a list of warninglists
[**search_warninglists**](WarninglistsApi.md#search_warninglists) | **POST** /warninglists | Search warninglists
[**toggle_enable_warninglist**](WarninglistsApi.md#toggle_enable_warninglist) | **POST** /warninglists/toggleEnable | Enable/disable warninglists
[**update_warninglists**](WarninglistsApi.md#update_warninglists) | **POST** /warninglists/update | Update warninglists



## check_value_warninglists_matches

> serde_json::Value check_value_warninglists_matches(request_body)
Check if a list of values matches any warninglists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_warninglist_by_id

> models::GetWarninglists200ResponseWarninglistsInner get_warninglist_by_id(warninglist_id)
Get warninglist by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**warninglist_id** | **String** | Numeric ID of the warninglist | [required] |

### Return type

[**models::GetWarninglists200ResponseWarninglistsInner**](getWarninglists_200_response_Warninglists_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_warninglists

> models::GetWarninglists200Response get_warninglists()
Get a list of warninglists

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWarninglists200Response**](getWarninglists_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_warninglists

> models::GetWarninglists200Response search_warninglists(value, enabled)
Search warninglists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | Option<**String**> | Search term to be used to match warninglists name, description or type. |  |
**enabled** | Option<**bool**> |  |  |

### Return type

[**models::GetWarninglists200Response**](getWarninglists_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_enable_warninglist

> models::ToggleEnableWarninglist200Response toggle_enable_warninglist(id, name, enabled)
Enable/disable warninglists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<String>**](String.md)> | One or more warninglist IDs. For a single ID, provide an array with one element. |  |
**name** | Option<[**Vec<String>**](String.md)> | One or more warninglist name patterns (e.g., \\\"%search term%\\\"). For a single pattern, provide an array with one element. |  |
**enabled** | Option<**bool**> |  |  |

### Return type

[**models::ToggleEnableWarninglist200Response**](toggleEnableWarninglist_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_warninglists

> models::UpdateWarninglists200Response update_warninglists()
Update warninglists

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UpdateWarninglists200Response**](updateWarninglists_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

