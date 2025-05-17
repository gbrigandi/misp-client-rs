# SearchEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page** | Option<**i32**> |  | [optional]
**limit** | Option<**i32**> |  | [optional]
**sort** | Option<**String**> | Field to be used to sort the result | [optional]
**direction** | Option<[**models::DirectionSearchField**](DirectionSearchField.md)> |  | [optional]
**minimal** | Option<**bool**> | Returns a minimal version of the event, only events with `attributeCount` > 0 will be returned | [optional][default to false]
**attribute** | Option<**String**> | Filter events matching the given string with attributes values | [optional]
**eventid** | Option<**String**> |  | [optional]
**datefrom** | Option<[**String**](string.md)> | Event creation date is greater or equal | [optional]
**dateuntil** | Option<[**String**](string.md)> | Event creation date is less or equal | [optional]
**org** | Option<**String**> | Filter events by matching the creator organisation name | [optional]
**eventinfo** | Option<**String**> | Filter events by matching the event info text | [optional]
**tag** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> | Filter events by matching *any* of the event tags of a given list of tag names | [optional]
**distribution** | Option<[**models::DistributionLevelId**](DistributionLevelId.md)> |  | [optional]
**sharinggroup** | Option<**String**> |  | [optional]
**analysis** | Option<[**models::AnalysisLevelId**](AnalysisLevelId.md)> |  | [optional]
**threatlevel** | Option<[**models::ThreatLevelId**](ThreatLevelId.md)> |  | [optional]
**email** | Option<**String**> | Filter events by matching the event creator user email | [optional]
**hasproposal** | Option<**String**> | Filter events by checking if it has attributes with change proposals. Possible values: `0`, `1` | [optional]
**timestamp** | Option<**String**> | Event timestamp greater or equal | [optional]
**publish_timestamp** | Option<**String**> | Event publish timestamp greater or equal | [optional]
**search_datefrom** | Option<**String**> | Filters on the date, anything newer than the given date in YYYY-MM-DD format is taken - non-negatable | [optional]
**search_dateuntil** | Option<**String**> | Filters on the date, anything older than the given date in YYYY-MM-DD format is taken - non-negatable | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


