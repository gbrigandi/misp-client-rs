# \TagsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_tag**](TagsApi.md#add_tag) | **POST** /tags/add | Add tag
[**delete_tag**](TagsApi.md#delete_tag) | **POST** /tags/delete/{tagId} | Delete tag
[**edit_tag**](TagsApi.md#edit_tag) | **POST** /tags/edit/{tagId} | Edit tag
[**get_tag_by_id**](TagsApi.md#get_tag_by_id) | **GET** /tags/view/{tagId} | Get tag by ID
[**get_tags**](TagsApi.md#get_tags) | **GET** /tags | Get tags
[**search_tag**](TagsApi.md#search_tag) | **GET** /tags/search/{tagSearchTerm} | Search tag



## add_tag

> models::Tag add_tag(tag_no_id)
Add tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_no_id** | Option<[**TagNoId**](TagNoId.md)> |  |  |

### Return type

[**models::Tag**](Tag.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tag

> models::DeleteTag200Response delete_tag(tag_id)
Delete tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | Numeric ID of the attribute | [required] |

### Return type

[**models::DeleteTag200Response**](deleteTag_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_tag

> models::EditTag200Response edit_tag(tag_id, tag_no_id)
Edit tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | Numeric ID of the attribute | [required] |
**tag_no_id** | Option<[**TagNoId**](TagNoId.md)> |  |  |

### Return type

[**models::EditTag200Response**](editTag_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tag_by_id

> models::Tag get_tag_by_id(tag_id)
Get tag by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_id** | **String** | Numeric ID of the attribute | [required] |

### Return type

[**models::Tag**](Tag.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags

> models::GetTags200Response get_tags()
Get tags

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTags200Response**](getTags_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tag

> Vec<models::ExtendedTag> search_tag(tag_search_term)
Search tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_search_term** | **String** | Tag search term | [required] |

### Return type

[**Vec<models::ExtendedTag>**](ExtendedTag.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

