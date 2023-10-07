# SFM (Sync File Mover)

    Is a CLI Program to move files from a -> b, when defined filters are fulfilled
___
## Explanation:
You start with --src **FOLDER_PATH_WITHOUT_SPACES** --target **FOLDER_PATH_WITHOUT_SPACES**

Then you can define your filters, and to finish it off, you write **--new**
___
## Filters:

| Filter             | Usage                                       | Explanation                                                                                                    |
|--------------------|---------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| --starts_with      | --starts_with {**WORD**}                    | checks if before the . defining the extension the **filename starts** with the specified word                  |
| --ends_with        | --ends_with {**WORD**}                      | checks if before the . defining the extension the **filename ends** with the specified word                    |
| --ends_with_ext    | --ends_with_ext {**EXTENSION**}             | checks if after the . defining the **extension is the specified word**                                         |
| --contains         | --contains {**WORD**}                       | checks if before the . defining the extension the filename **contains** the specified word                     |
| --contains_x_times | --contains_x_times {**WORD**} {**INTEGER**} | checks if before the . defining the extension the filename **contains** the specified word exactly **x** times |
| --exact            | --exact {**WORD**}                          | checks if the full file name is exactly the specified word                                                     |
| --invert           | --invert {**BEFORE OTHER FILTERS**}         | **inverts** the **final result** of the filter (can be combined with match_all)                                |
| --match_all        | --match_all {**BEFORE OTHER FILTERS**}      | changes the any (**or**) of the filter **into** an all (**and**) (can be combined with invert)                 |

___
## Example Usage:

`sfm.exe --src C:\tmp\folderA --target D:\Downloads --invert --ends_with_ext tmp --new`

## Download:
You can find the most recent compiled version under Actions, selecting it you can choose your Architecture and download from the provided gofile.io link.