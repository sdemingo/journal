# Journal

Journal is a simple app to manage my journals and life logs files. From years
I've been using a simple format to store what happened in a day. 


## My journal files

With this command I can extract information from this logs file. This app
expect your journals directory as:

```
$JOURNALPATH
     |
     |------ 2022
     |       |------ 2022-01.md
     |       |------ 2022-02.md
     |       |------ 2022-03.md
     |       |------ one file per month
     |
     |------ 2021
     |------ 2020
     |------ one dir per year

```

A month file looks like:

```
# March

## Monday - March 1st

Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis
facilisis, est dui fermentum leo, quis tempor ligula erat quis odio.  Nunc porta
vulputate tellus.  Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada massa, quis
varius mi purus non odio.  Pellentesque condimentum, magna ut suscipit
hendrerit, ipsum augue ornare nulla, non luctus diam neque sit amet urna.
Curabitur vulputate vestibulum lorem.  Fusce sagittis, libero non molestie
mollis, magna orci ultrices dolor, at vulputate neque nulla lacinia eros.  Sed
id ligula quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a
sapien.

tag1: Aenean in sem ac leo mollis blandit.
tag2: Donec neque quam, dignissim in, mollis nec, sagittis eu, wisi.  

## Tuesday - March 2nd

....

```

The first one line paragraph is a summary day. Next, I use tags (work, coding,
friends, etc.) to remark special events about something.



## Usage

First, you have to set `JOURNALPATH` variable in your environment with the path
of your journals directory.

You can use `journal` with nexts options:

* `-d dd/mm/yy`: Show the entry for this day.
* `-n N`: Show the nexts `N` entries from `-d` option.
* `-t N`: Show today with `N=0` or the `N` previous days.
* `-p pattern`: Show entries that contains the pattern. You can combine with `-m` or `-y`.
* `-t tag`: Show the tags entries in current month. You can combine with `-m` or `-y`. 





