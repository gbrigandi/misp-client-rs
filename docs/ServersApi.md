# \ServersApi

All URIs are relative to *https://misp.local*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_server**](ServersApi.md#add_server) | **POST** /servers/add | Add server
[**cache_server**](ServersApi.md#cache_server) | **POST** /servers/cache | Cache server
[**create_sync**](ServersApi.md#create_sync) | **POST** /servers/createSync | Create sync
[**delete_server**](ServersApi.md#delete_server) | **POST** /servers/delete/{serverId} | Delete server
[**edit_server**](ServersApi.md#edit_server) | **PUT** /servers/edit/{serverId} | Edit server
[**edit_server_setting**](ServersApi.md#edit_server_setting) | **POST** /servers/serverSettingsEdit/{settingName} | Edit server setting
[**get_py_misp_version**](ServersApi.md#get_py_misp_version) | **GET** /servers/getPyMISPVersion | Get current instance PyMISP version
[**get_server_setting**](ServersApi.md#get_server_setting) | **GET** /servers/getSetting/{settingName} | Get server setting by name
[**get_server_settings**](ServersApi.md#get_server_settings) | **GET** /servers/serverSettings | Get current instance settings and diagnostics
[**get_server_uuid**](ServersApi.md#get_server_uuid) | **GET** /servers/getInstanceUUID | Get instance UUID
[**get_server_version**](ServersApi.md#get_server_version) | **GET** /servers/getVersion | Get current instance version
[**get_servers**](ServersApi.md#get_servers) | **GET** /servers | Get servers
[**get_workers**](ServersApi.md#get_workers) | **GET** /servers/getWorkers | Get workers
[**import_server**](ServersApi.md#import_server) | **POST** /servers/import | Import server
[**kill_all_workers**](ServersApi.md#kill_all_workers) | **POST** /servers/killAllWorkers | Kill all workers
[**pull_server**](ServersApi.md#pull_server) | **GET** /servers/pull/{serverId}/{pullTechnique} | Pull server
[**push_server**](ServersApi.md#push_server) | **GET** /servers/push/{serverId}/{pushTechnique} | Push server
[**restart_dead_workers**](ServersApi.md#restart_dead_workers) | **POST** /servers/restartDeadWorkers | Restart dead workers
[**restart_workers**](ServersApi.md#restart_workers) | **POST** /servers/restartWorkers | Restart workers
[**start_worker**](ServersApi.md#start_worker) | **POST** /servers/startWorker/{workerType} | Start worker
[**stop_worker**](ServersApi.md#stop_worker) | **POST** /servers/stopWorker/{workerPid} | Stop worker
[**update_server**](ServersApi.md#update_server) | **POST** /servers/update | Update server



## add_server

> models::AddServer200Response add_server(server_no_id)
Add server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_no_id** | Option<[**ServerNoId**](ServerNoId.md)> |  |  |

### Return type

[**models::AddServer200Response**](addServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cache_server

> models::CacheServer200Response cache_server()
Cache server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CacheServer200Response**](cacheServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sync

> models::CreateSync200Response create_sync()
Create sync

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateSync200Response**](createSync_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_server

> models::DeleteServer200Response delete_server(server_id)
Delete server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** | UUID or numeric ID of the server | [required] |

### Return type

[**models::DeleteServer200Response**](deleteServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_server

> models::AddServer200Response edit_server(server_id, server)
Edit server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** | UUID or numeric ID of the server | [required] |
**server** | Option<[**Server**](Server.md)> |  |  |

### Return type

[**models::AddServer200Response**](addServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_server_setting

> models::EditServerSetting200Response edit_server_setting(setting_name, edit_server_setting_request)
Edit server setting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | Setting name. | [required] |
**edit_server_setting_request** | Option<[**EditServerSettingRequest**](EditServerSettingRequest.md)> |  |  |

### Return type

[**models::EditServerSetting200Response**](editServerSetting_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_py_misp_version

> models::GetPyMispVersion200Response get_py_misp_version()
Get current instance PyMISP version

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPyMispVersion200Response**](getPyMISPVersion_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_setting

> models::MispSetting get_server_setting(setting_name)
Get server setting by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**setting_name** | **String** | Setting name. | [required] |

### Return type

[**models::MispSetting**](MispSetting.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_settings

> models::GetServerSettings200Response get_server_settings()
Get current instance settings and diagnostics

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetServerSettings200Response**](getServerSettings_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_uuid

> models::GetServerUuid200Response get_server_uuid()
Get instance UUID

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetServerUuid200Response**](getServerUuid_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_version

> models::GetServerVersion200Response get_server_version()
Get current instance version

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetServerVersion200Response**](getServerVersion_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_servers

> Vec<models::ServerListItem> get_servers()
Get servers

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ServerListItem>**](ServerListItem.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workers

> models::GetServerSettings200ResponseWorkers get_workers()
Get workers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetServerSettings200ResponseWorkers**](getServerSettings_200_response_workers.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## import_server

> models::AddServer200Response import_server(import_server_request)
Import server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_server_request** | Option<[**ImportServerRequest**](ImportServerRequest.md)> |  |  |

### Return type

[**models::AddServer200Response**](addServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## kill_all_workers

> models::KillAllWorkers200Response kill_all_workers()
Kill all workers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::KillAllWorkers200Response**](killAllWorkers_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_server

> models::PullServer200Response pull_server(server_id, pull_technique)
Pull server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** | UUID or numeric ID of the server | [required] |
**pull_technique** | **String** | Pull technique to be used for pulling events from this instance. | [required] |

### Return type

[**models::PullServer200Response**](pullServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_server

> models::PushServer200Response push_server(server_id, push_technique)
Push server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**server_id** | **String** | UUID or numeric ID of the server | [required] |
**push_technique** | **String** | Push technique to be used for pushing events to this instance. | [required] |

### Return type

[**models::PushServer200Response**](pushServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_dead_workers

> models::RestartDeadWorkers200Response restart_dead_workers()
Restart dead workers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestartDeadWorkers200Response**](restartDeadWorkers_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_workers

> models::RestartWorkers200Response restart_workers()
Restart workers

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RestartWorkers200Response**](restartWorkers_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_worker

> models::StartWorker200Response start_worker(worker_type)
Start worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_type** | **String** | Worker type. | [required] |

### Return type

[**models::StartWorker200Response**](startWorker_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_worker

> models::StopWorker200Response stop_worker(worker_pid)
Stop worker

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_pid** | **String** | Worker PID. | [required] |

### Return type

[**models::StopWorker200Response**](stopWorker_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_server

> models::UpdateServer200Response update_server()
Update server

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UpdateServer200Response**](updateServer_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

