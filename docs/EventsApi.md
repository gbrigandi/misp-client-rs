# \EventsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_event**](EventsApi.md#add_event) | **POST** /events/add | Add event
[**delete_event**](EventsApi.md#delete_event) | **DELETE** /events/delete/{eventId} | Delete event
[**edit_event**](EventsApi.md#edit_event) | **PUT** /events/edit/{eventId} | Edit event
[**enrich_event**](EventsApi.md#enrich_event) | **POST** /events/enrichEvent/{eventId} | Enrich an event with the given modules
[**get_event_by_id**](EventsApi.md#get_event_by_id) | **GET** /events/view/{eventId} | Get event by ID
[**get_events**](EventsApi.md#get_events) | **GET** /events | Get a list of events
[**publish_event**](EventsApi.md#publish_event) | **POST** /events/publish/{eventId} | Publish an event
[**rest_search_events**](EventsApi.md#rest_search_events) | **POST** /events/restSearch | [restSearch] Get a filtered and paginated list of events
[**search_events**](EventsApi.md#search_events) | **POST** /events/index | Search events
[**tag_event**](EventsApi.md#tag_event) | **POST** /events/addTag/{eventId}/{tagId}/local:{local} | Add event tag
[**unpublish_event**](EventsApi.md#unpublish_event) | **POST** /events/unpublish/{eventId} | Unpublish an event
[**untag_event**](EventsApi.md#untag_event) | **POST** /events/removeTag/{eventId}/{tagId} | Remove event tag



## add_event

> models::EditedEvent add_event(event_no_id)
Add event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_no_id** | [**EventNoId**](EventNoId.md) |  | [required] |

### Return type

[**models::EditedEvent**](EditedEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event

> models::DeleteEvent200Response delete_event(event_id)
Delete event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |

### Return type

[**models::DeleteEvent200Response**](deleteEvent_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_event

> models::EditedEvent edit_event(event_id, event_no_id)
Edit event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**event_no_id** | [**EventNoId**](EventNoId.md) |  | [required] |

### Return type

[**models::EditedEvent**](EditedEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrich_event

> models::EnrichEvent200Response enrich_event(event_id, enrich_modules_list)
Enrich an event with the given modules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**enrich_modules_list** | [**EnrichModulesList**](EnrichModulesList.md) |  | [required] |

### Return type

[**models::EnrichEvent200Response**](enrichEvent_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_by_id

> models::GetEventById200Response get_event_by_id(event_id)
Get event by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |

### Return type

[**models::GetEventById200Response**](getEventById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_events

> Vec<models::ExtendedEvent> get_events()
Get a list of events

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ExtendedEvent>**](ExtendedEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_event

> models::PublishEvent200Response publish_event(event_id)
Publish an event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |

### Return type

[**models::PublishEvent200Response**](publishEvent_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rest_search_events

> models::RestSearchEvents200Response rest_search_events(rest_search_events_request)
[restSearch] Get a filtered and paginated list of events

**This is the recommended endpoint for searching events.** 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rest_search_events_request** | [**RestSearchEventsRequest**](RestSearchEventsRequest.md) |  | [required] |

### Return type

[**models::RestSearchEvents200Response**](restSearchEvents_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_events

> Vec<models::ExtendedEvent> search_events(search_events_request)
Search events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_events_request** | [**SearchEventsRequest**](SearchEventsRequest.md) |  | [required] |

### Return type

[**Vec<models::ExtendedEvent>**](ExtendedEvent.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tag_event

> models::TagAttribute200Response tag_event(event_id, tag_id, local)
Add event tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**tag_id** | **String** | Numeric ID of the attribute | [required] |
**local** | **bool** | Whether the object should be attached locally or not to the target | [required] |

### Return type

[**models::TagAttribute200Response**](tagAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_event

> models::UnpublishEvent200Response unpublish_event(event_id)
Unpublish an event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |

### Return type

[**models::UnpublishEvent200Response**](unpublishEvent_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## untag_event

> models::UntagAttribute200Response untag_event(event_id, tag_id)
Remove event tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | UUID or numeric ID of the event | [required] |
**tag_id** | **String** | Numeric ID of the attribute | [required] |

### Return type

[**models::UntagAttribute200Response**](untagAttribute_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

