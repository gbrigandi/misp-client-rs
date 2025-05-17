# \AttributesApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_attribute**](AttributesApi.md#add_attribute) | **POST** /attributes/add/{eventId} | Add an attribute
[**delete_attribute**](AttributesApi.md#delete_attribute) | **DELETE** /attributes/delete/{attributeId} | Delete an attribute
[**describe_attribute_types**](AttributesApi.md#describe_attribute_types) | **GET** /attributes/describeTypes | Get a list of the available attribute types
[**edit_attribute**](AttributesApi.md#edit_attribute) | **PUT** /attributes/edit/{attributeId} | Edit an attribute
[**enrich_attribute**](AttributesApi.md#enrich_attribute) | **POST** /attributes/enrich/{attributeId} | Enrich an attribute with the given modules
[**get_attribute_by_id**](AttributesApi.md#get_attribute_by_id) | **GET** /attributes/view/{attributeId} | Get an attribute by ID
[**get_attribute_statistics**](AttributesApi.md#get_attribute_statistics) | **GET** /attributes/attributeStatistics/{context}/{percentage} | Get the count of attributes per category
[**get_attributes**](AttributesApi.md#get_attributes) | **GET** /attributes | Get a list of attributes
[**rest_search_attributes**](AttributesApi.md#rest_search_attributes) | **POST** /attributes/restSearch | [restSearch] Get a filtered and paginated list of attributes
[**restore_attribute**](AttributesApi.md#restore_attribute) | **POST** /attributes/restore/{attributeId} | Restore an attribute
[**tag_attribute**](AttributesApi.md#tag_attribute) | **POST** /attributes/addTag/{attributeId}/{tagId}/local:{local} | Add tag(s) to attribute(s)
[**untag_attribute**](AttributesApi.md#untag_attribute) | **POST** /attributes/removeTag/{attributeId}/{tagId} | Remove a tag from an attribute



## add_attribute

> models::AddAttribute200Response add_attribute(event_id, attribute_no_id)
Add an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**attribute_no_id** | [**AttributeNoId**](AttributeNoId.md) |  | [required] |

### Return type

[**models::AddAttribute200Response**](addAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_attribute

> models::DeleteAttribute200Response delete_attribute(attribute_id)
Delete an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |

### Return type

[**models::DeleteAttribute200Response**](deleteAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## describe_attribute_types

> models::DescribeAttributeTypesResponse describe_attribute_types()
Get a list of the available attribute types

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DescribeAttributeTypesResponse**](DescribeAttributeTypesResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_attribute

> models::AddAttribute200Response edit_attribute(attribute_id, attribute)
Edit an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |
**attribute** | [**Attribute**](Attribute.md) |  | [required] |

### Return type

[**models::AddAttribute200Response**](addAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrich_attribute

> models::EnrichAttribute200Response enrich_attribute(attribute_id, enrich_modules_list)
Enrich an attribute with the given modules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |
**enrich_modules_list** | [**EnrichModulesList**](EnrichModulesList.md) |  | [required] |

### Return type

[**models::EnrichAttribute200Response**](enrichAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attribute_by_id

> models::AddAttribute200Response get_attribute_by_id(attribute_id)
Get an attribute by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |

### Return type

[**models::AddAttribute200Response**](addAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attribute_statistics

> serde_json::Value get_attribute_statistics(context, percentage)
Get the count of attributes per category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | **String** | Context of the statistics. | [required] |[default to type]
**percentage** | **i32** | `0` to show attribute count, `1` for showing percentages | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_attributes

> Vec<models::Attribute> get_attributes()
Get a list of attributes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Attribute>**](Attribute.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_search_attributes

> models::RestSearchAttributes200Response rest_search_attributes(attribute_rest_search_filter)
[restSearch] Get a filtered and paginated list of attributes

**This is the recommended endpoint for searching attributes.** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_rest_search_filter** | [**AttributeRestSearchFilter**](AttributeRestSearchFilter.md) |  | [required] |

### Return type

[**models::RestSearchAttributes200Response**](restSearchAttributes_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_attribute

> models::AddAttribute200Response restore_attribute(attribute_id)
Restore an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |

### Return type

[**models::AddAttribute200Response**](addAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_attribute

> models::TagAttribute200Response tag_attribute(attribute_id, tag_id, local, tag_attribute_request)
Add tag(s) to attribute(s)

Add one or multiple tags to one or multiple attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | Attribute ID or 'selected' to add tags to multiple attributes | [required] |
**tag_id** | **String** | Tag ID, tag name or tag collection ID (format collection_N) | [required] |
**local** | **i32** | Whether to add tag as local tag (1) or global tag (0). Leave the other path params empty if you want to add local tags using the json request body params | [required] |[default to 0]
**tag_attribute_request** | Option<[**TagAttributeRequest**](TagAttributeRequest.md)> |  |  |

### Return type

[**models::TagAttribute200Response**](tagAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## untag_attribute

> models::UntagAttribute200Response untag_attribute(attribute_id, tag_id)
Remove a tag from an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |
**tag_id** | **String** | Numeric ID of the attribute | [required] |

### Return type

[**models::UntagAttribute200Response**](untagAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

