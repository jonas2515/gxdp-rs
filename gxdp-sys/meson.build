project(
    'gxdp',
    'c',
    version: '1.0.0',
)

subproject(
    'libgxdp',
    default_options: 'tests=false'
)

static_library(
    'gxdp',
    dependencies: dependency('libgxdp'),
)