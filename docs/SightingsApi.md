# \SightingsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_sighting**](SightingsApi.md#add_sighting) | **POST** /sightings/add/{attributeId} | Add sighting of an attribute
[**add_sighting_by_value**](SightingsApi.md#add_sighting_by_value) | **POST** /sightings/add | Add sightings of a list of values
[**delete_sighting**](SightingsApi.md#delete_sighting) | **POST** /sightings/delete/{sightingId} | Delete sighting
[**get_sightings_by_event_id**](SightingsApi.md#get_sightings_by_event_id) | **GET** /sightings/index/{eventId} | Get sightings by event ID



## add_sighting

> models::Sighting add_sighting(attribute_id)
Add sighting of an attribute

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attribute_id** | **String** | UUID or numeric ID of the attribute | [required] |

### Return type

[**models::Sighting**](Sighting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_sighting_by_value

> models::Sighting add_sighting_by_value(add_sighting_by_value_request)
Add sightings of a list of values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_sighting_by_value_request** | Option<[**AddSightingByValueRequest**](AddSightingByValueRequest.md)> |  |  |

### Return type

[**models::Sighting**](Sighting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sighting

> models::DeleteSighting200Response delete_sighting(sighting_id)
Delete sighting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sighting_id** | **String** | UUID or numeric ID of the sighting | [required] |

### Return type

[**models::DeleteSighting200Response**](deleteSighting_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sightings_by_event_id

> Vec<models::Sighting> get_sightings_by_event_id(event_id)
Get sightings by event ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |

### Return type

[**Vec<models::Sighting>**](Sighting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

