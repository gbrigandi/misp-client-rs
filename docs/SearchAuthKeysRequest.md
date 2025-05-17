# SearchAuthKeysRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> |  | [optional]
**limit** | Option<**i32**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**authkey_start** | Option<**String**> | Search term matching the first 4 characers of the authkey | [optional]
**authkey_end** | Option<**String**> | Search term matching the last 4 characers of the authkey | [optional]
**created** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]
**expiration** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]
**read_only** | Option<**bool**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**allowed_ips** | Option<**String**> | Stringified JSON array of the IP addresses. | [optional]
**last_used** | Option<**String**> | You can use any of the valid time related filters (examples: 7d, timestamps, [14d, 7d] for ranges, etc.) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


