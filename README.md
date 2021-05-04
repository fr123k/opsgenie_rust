# opsgenie_rust

This is a simple proof of concept for exporting schedule timelines from opsgenie and generating an kind of report for reimbursement purpose of Human Resource department.

The report will be in a csv format.

# Todo
* [x] build internal data structure with names and dates
* [x] convert to CEST timezone
* [ ] count nights and weekends (week day night +1, weekend day +1 and night+1)
* [x] check base and overide schedule (if shifts were changed) or use the final schedule but check for invalid entries
* [ ] save the csv report in a file instead of stdout