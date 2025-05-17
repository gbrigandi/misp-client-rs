# \SharingGroupsApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organisation_to_sharing_group**](SharingGroupsApi.md#add_organisation_to_sharing_group) | **POST** /sharing_groups/addOrg/{sharingGroupId}/{organisationId} | Add an organisation to a sharing group
[**add_server_to_sharing_group**](SharingGroupsApi.md#add_server_to_sharing_group) | **POST** /sharing_groups/addServer/{sharingGroupId}/{serverId} | Add a server to a sharing group
[**add_sharing_group**](SharingGroupsApi.md#add_sharing_group) | **POST** /sharing_groups/add | Add a sharing group
[**delete_sharing_group**](SharingGroupsApi.md#delete_sharing_group) | **DELETE** /sharing_groups/delete/{sharingGroupId} | Delete a sharing group
[**edit_sharing_group**](SharingGroupsApi.md#edit_sharing_group) | **POST** /sharing_groups/edit/{sharingGroupId} | Edit a sharing group
[**get_sharing_group**](SharingGroupsApi.md#get_sharing_group) | **GET** /sharing_groups | Get a list of sharing groups
[**get_sharing_group_by_id**](SharingGroupsApi.md#get_sharing_group_by_id) | **GET** /sharing_groups/view/{sharingGroupId} | Get a sharing group by ID
[**remove_organisation_from_sharing_group**](SharingGroupsApi.md#remove_organisation_from_sharing_group) | **POST** /sharing_groups/removeOrg/{sharingGroupId}/{organisationId} | Remove an organisation from a sharing group
[**remove_server_from_sharing_group**](SharingGroupsApi.md#remove_server_from_sharing_group) | **POST** /sharing_groups/removeServer/{sharingGroupServerId}/{serverId} | Remove a server from a sharing group



## add_organisation_to_sharing_group

> models::AddOrganisationToSharingGroup200Response add_organisation_to_sharing_group(sharing_group_id, organisation_id)
Add an organisation to a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |
**organisation_id** | **String** | UUID or numeric ID of the organisation | [required] |

### Return type

[**models::AddOrganisationToSharingGroup200Response**](addOrganisationToSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_server_to_sharing_group

> models::AddServerToSharingGroup200Response add_server_to_sharing_group(sharing_group_id, server_id)
Add a server to a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |
**server_id** | **String** | UUID or numeric ID of the server | [required] |

### Return type

[**models::AddServerToSharingGroup200Response**](addServerToSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_sharing_group

> models::AddSharingGroup200Response add_sharing_group(sharing_group_no_id)
Add a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_no_id** | Option<[**SharingGroupNoId**](SharingGroupNoId.md)> |  |  |

### Return type

[**models::AddSharingGroup200Response**](addSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sharing_group

> models::DeleteSharingGroup200Response delete_sharing_group(sharing_group_id)
Delete a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |

### Return type

[**models::DeleteSharingGroup200Response**](deleteSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_sharing_group

> models::AddSharingGroup200Response edit_sharing_group(sharing_group_id, sharing_group)
Edit a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |
**sharing_group** | Option<[**SharingGroup**](SharingGroup.md)> |  |  |

### Return type

[**models::AddSharingGroup200Response**](addSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sharing_group

> models::GetSharingGroup200Response get_sharing_group()
Get a list of sharing groups

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSharingGroup200Response**](getSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sharing_group_by_id

> models::AddSharingGroup200Response get_sharing_group_by_id(sharing_group_id)
Get a sharing group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |

### Return type

[**models::AddSharingGroup200Response**](addSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_organisation_from_sharing_group

> models::RemoveOrganisationFromSharingGroup200Response remove_organisation_from_sharing_group(sharing_group_id, organisation_id)
Remove an organisation from a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_id** | **String** | UUID or numeric ID of the sharing group | [required] |
**organisation_id** | **String** | UUID or numeric ID of the organisation | [required] |

### Return type

[**models::RemoveOrganisationFromSharingGroup200Response**](removeOrganisationFromSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_server_from_sharing_group

> models::RemoveServerFromSharingGroup200Response remove_server_from_sharing_group(sharing_group_server_id, server_id)
Remove a server from a sharing group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_group_server_id** | Option<**String**> | UUID or numeric ID of the sharing group server | [required] |
**server_id** | **String** | UUID or numeric ID of the server | [required] |

### Return type

[**models::RemoveServerFromSharingGroup200Response**](removeServerFromSharingGroup_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

