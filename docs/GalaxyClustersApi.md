# \GalaxyClustersApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_galaxy_cluster**](GalaxyClustersApi.md#add_galaxy_cluster) | **POST** /galaxy_clusters/add/{galaxyId} | Add galaxy cluster
[**delete_galaxy_cluster**](GalaxyClustersApi.md#delete_galaxy_cluster) | **POST** /galaxy_clusters/delete/{galaxyClusterId} | Delete galaxy cluster
[**edit_galaxy_cluster**](GalaxyClustersApi.md#edit_galaxy_cluster) | **PUT** /galaxy_clusters/edit/{galaxyClusterId} | Edit galaxy cluster
[**get_galaxy_cluster_by_id**](GalaxyClustersApi.md#get_galaxy_cluster_by_id) | **GET** /galaxy_clusters/view/{galaxyClusterId} | Get galaxy cluster by ID
[**get_galaxy_clusters**](GalaxyClustersApi.md#get_galaxy_clusters) | **GET** /galaxy_clusters/index/{galaxyId} | Get galaxy clusters
[**publish_galaxy_cluster**](GalaxyClustersApi.md#publish_galaxy_cluster) | **POST** /galaxy_clusters/publish/{galaxyClusterId} | Publish galaxy cluster
[**restore_galaxy_cluster**](GalaxyClustersApi.md#restore_galaxy_cluster) | **POST** /galaxy_clusters/restore/{galaxyClusterId} | Restore galaxy cluster
[**search_galaxy_clusters**](GalaxyClustersApi.md#search_galaxy_clusters) | **POST** /galaxy_clusters/index/{galaxyId} | Search galaxy clusters
[**unpublish_galaxy_cluster**](GalaxyClustersApi.md#unpublish_galaxy_cluster) | **POST** /galaxy_clusters/unpublish/{galaxyClusterId} | Unpublish galaxy cluster



## add_galaxy_cluster

> models::AddGalaxyCluster200Response add_galaxy_cluster(galaxy_id, galaxy_cluster)
Add galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |
**galaxy_cluster** | Option<[**GalaxyCluster**](GalaxyCluster.md)> |  |  |

### Return type

[**models::AddGalaxyCluster200Response**](addGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_galaxy_cluster

> models::DeleteGalaxyCluster200Response delete_galaxy_cluster(galaxy_cluster_id)
Delete galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |

### Return type

[**models::DeleteGalaxyCluster200Response**](deleteGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_galaxy_cluster

> models::AddGalaxyCluster200Response edit_galaxy_cluster(galaxy_cluster_id, galaxy_cluster)
Edit galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |
**galaxy_cluster** | Option<[**GalaxyCluster**](GalaxyCluster.md)> |  |  |

### Return type

[**models::AddGalaxyCluster200Response**](addGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_galaxy_cluster_by_id

> models::GetGalaxyClusterById200Response get_galaxy_cluster_by_id(galaxy_cluster_id)
Get galaxy cluster by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |

### Return type

[**models::GetGalaxyClusterById200Response**](getGalaxyClusterById_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_galaxy_clusters

> Vec<models::AddGalaxyCluster200Response> get_galaxy_clusters(galaxy_id)
Get galaxy clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |

### Return type

[**Vec<models::AddGalaxyCluster200Response>**](addGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_galaxy_cluster

> models::PublishGalaxyCluster200Response publish_galaxy_cluster(galaxy_cluster_id)
Publish galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |

### Return type

[**models::PublishGalaxyCluster200Response**](publishGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_galaxy_cluster

> models::RestoreGalaxyCluster200Response restore_galaxy_cluster(galaxy_cluster_id)
Restore galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |

### Return type

[**models::RestoreGalaxyCluster200Response**](restoreGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_galaxy_clusters

> Vec<models::AddGalaxyCluster200Response> search_galaxy_clusters(galaxy_id, search_galaxy_clusters_request)
Search galaxy clusters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_id** | **String** | UUID or numeric ID of the galaxy | [required] |
**search_galaxy_clusters_request** | Option<[**SearchGalaxyClustersRequest**](SearchGalaxyClustersRequest.md)> |  |  |

### Return type

[**Vec<models::AddGalaxyCluster200Response>**](addGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpublish_galaxy_cluster

> models::UnpublishGalaxyCluster200Response unpublish_galaxy_cluster(galaxy_cluster_id)
Unpublish galaxy cluster

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**galaxy_cluster_id** | **String** | UUID or numeric ID of the galaxy cluster | [required] |

### Return type

[**models::UnpublishGalaxyCluster200Response**](unpublishGalaxyCluster_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

