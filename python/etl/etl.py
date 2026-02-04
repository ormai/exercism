def transform(legacy_data):
    return {v.lower(): k for k in legacy_data for v in legacy_data[k]}
