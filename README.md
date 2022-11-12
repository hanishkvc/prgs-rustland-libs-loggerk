# A simple logger library

Auther: HanishKVC

Version: 20220825IST0644

License: LGPL-3.0+

## Overview


### the trait and family

It defines a logger trait, which allows one to categorise messages and
inturn enable or disable them at runtime.

Its mechanism also allows different targets to be implemented wrt the
logged messages, without needing to modify the logics(programs/libraries)
which may be using this logger library.

The supported message categories are

* error
* warning
* info
* others
* dbug

Currently console target has been implemented. However other targets can
be easily added by implementing the logger trait wrt a given target.

### macro

A macro called ldebug! is defined, which generates code to call log_d with
the string data passed to it, but only in debug builds. In release builds,
no code is generated.

