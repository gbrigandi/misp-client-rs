# AnalystOpinion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**object_type** | Option<[**models::AnalystObjectType**](AnalystObjectType.md)> |  | [optional]
**authors** | Option<**String**> |  | [optional]
**org_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**orgc_uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**modified** | Option<**String**> |  | [optional]
**distribution** | Option<[**models::DistributionLevelId**](DistributionLevelId.md)> |  | [optional]
**sharing_group_id** | Option<**String**> |  | [optional]
**locked** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**opinion** | Option<**i32**> | The opinion expressed on a scale from 0 to 100 where values < 50 are negatives, 50 is neutral and values > 50 are positives | [optional]
**note_type_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


