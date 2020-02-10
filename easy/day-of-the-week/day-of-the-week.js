var dayOfTheWeek = function (day, month, year) {
  return [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
  ][new Date(`${year}-${month}-${day}`).getDay()]
};