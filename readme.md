# SFM (Sync File Mover)

    Is a CLI Program to move files from a -> b, when defined filters are fulfilled
___
## Explanation
This program was made to move from a->b from multiple sets of folders with differing filters.

It will check every 30s through all defined sets, and for each set if there has been no noticeable change (through metadata) for 10 Minutes, the files defined in the filter will be moved.
___
## Usage:
You start with --src **FOLDER_PATH_WITHOUT_SPACES** --target **FOLDER_PATH_WITHOUT_SPACES**

Then you can define your filters, and to finish it off, you write **--new**

This way you have defined **one set**, you can **repeat this step** for as many sets as needed.
___
## Filters:

| Filter             | Usage                                       | Explanation                                                                                                                                                    |
|--------------------|---------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| --starts_with      | --starts_with {**WORD**}                    | checks if before the . defining the extension the **filename starts** with the specified word                                                                  |
| --ends_with        | --ends_with {**WORD**}                      | checks if before the . defining the extension the **filename ends** with the specified word                                                                    |
| --ends_with_ext    | --ends_with_ext {**EXTENSION**}             | checks if after the . defining the **extension is the specified word**                                                                                         |
| --contains         | --contains {**WORD**}                       | checks if before the . defining the extension the filename **contains** the specified word                                                                     |
| --contains_x_times | --contains_x_times {**WORD**} {**INTEGER**} | checks if before the . defining the extension the filename **contains** the specified word exactly **x** times                                                 |
| --exact            | --exact {**WORD**}                          | checks if the full file name is exactly the specified word. Is also necesarry for files/folders starting with a (.) and no ending (.) declaring the extension |
| --invert           | --invert {**BEFORE OTHER FILTERS**}         | **inverts** the **final result** of the filter (can be combined with match_all)                                                                                |
| --match_all        | --match_all {**BEFORE OTHER FILTERS**}      | changes the any (**or**) of the filter **into** an all (**and**) (can be combined with invert)                                                                 |

___
## Example Usage:

`sfm.exe --src C:\tmp\folderA --target D:\Downloads --invert --ends_with_ext tmp --new`

## Download:
You can find the most recent compiled version under Actions, selecting it you can choose your Architecture and download from the provided gofile.io link.