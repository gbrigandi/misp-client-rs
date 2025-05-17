# \NoticelistsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_noticelist_by_id**](NoticelistsApi.md#get_noticelist_by_id) | **GET** /noticelists/view/{noticelistId} | Get a noticelist by ID
[**get_noticelists**](NoticelistsApi.md#get_noticelists) | **GET** /noticelists | Get a list of noticelists
[**toggle_enable_noticelist**](NoticelistsApi.md#toggle_enable_noticelist) | **POST** /noticelists/toggleEnable/{noticelistId} | Enable/disable noticelist
[**update_noticelists**](NoticelistsApi.md#update_noticelists) | **POST** /noticelists/update | Update noticelists



## get_noticelist_by_id

> models::GetNoticelists200ResponseInner get_noticelist_by_id(noticelist_id)
Get a noticelist by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**noticelist_id** | **String** | Numeric ID of the noticelist | [required] |

### Return type

[**models::GetNoticelists200ResponseInner**](getNoticelists_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_noticelists

> Vec<models::GetNoticelists200ResponseInner> get_noticelists()
Get a list of noticelists

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::GetNoticelists200ResponseInner>**](getNoticelists_200_response_inner.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_enable_noticelist

> models::ToggleEnableNoticelist200Response toggle_enable_noticelist(noticelist_id)
Enable/disable noticelist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**noticelist_id** | **String** | Numeric ID of the noticelist | [required] |

### Return type

[**models::ToggleEnableNoticelist200Response**](toggleEnableNoticelist_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_noticelists

> models::UpdateNoticelists200Response update_noticelists()
Update noticelists

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UpdateNoticelists200Response**](updateNoticelists_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

