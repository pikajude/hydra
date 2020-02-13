table! {
    aggregateconstituents (aggregate, constituent) {
        aggregate -> Int4,
        constituent -> Int4,
    }
}

table! {
    buildinputs (id) {
        id -> Int4,
        build -> Nullable<Int4>,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        uri -> Nullable<Text>,
        revision -> Nullable<Text>,
        value -> Nullable<Text>,
        emailresponsible -> Int4,
        dependency -> Nullable<Int4>,
        path -> Nullable<Text>,
        sha256hash -> Nullable<Text>,
    }
}

table! {
    buildmetrics (build, name) {
        build -> Int4,
        name -> Text,
        unit -> Nullable<Text>,
        value -> Float8,
        project -> Text,
        jobset -> Text,
        job -> Text,
        timestamp -> Int4,
    }
}

table! {
    buildoutputs (build, name) {
        build -> Int4,
        name -> Text,
        path -> Text,
    }
}

table! {
    buildproducts (build, productnr) {
        build -> Int4,
        productnr -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        subtype -> Text,
        filesize -> Nullable<Int8>,
        sha1hash -> Nullable<Text>,
        sha256hash -> Nullable<Text>,
        path -> Nullable<Text>,
        name -> Text,
        defaultpath -> Nullable<Text>,
    }
}

table! {
    builds (id) {
        id -> Int4,
        finished -> Int4,
        timestamp -> Int4,
        project -> Text,
        jobset -> Text,
        job -> Text,
        nixname -> Nullable<Text>,
        description -> Nullable<Text>,
        drvpath -> Text,
        system -> Text,
        license -> Nullable<Text>,
        homepage -> Nullable<Text>,
        maintainers -> Nullable<Text>,
        maxsilent -> Nullable<Int4>,
        timeout -> Nullable<Int4>,
        ischannel -> Int4,
        iscurrent -> Nullable<Int4>,
        nixexprinput -> Nullable<Text>,
        nixexprpath -> Nullable<Text>,
        priority -> Int4,
        globalpriority -> Int4,
        starttime -> Nullable<Int4>,
        stoptime -> Nullable<Int4>,
        iscachedbuild -> Nullable<Int4>,
        buildstatus -> Nullable<Int4>,
        size -> Nullable<Int8>,
        closuresize -> Nullable<Int8>,
        releasename -> Nullable<Text>,
        keep -> Int4,
        notificationpendingsince -> Nullable<Int4>,
    }
}

table! {
    buildstepoutputs (build, stepnr, name) {
        build -> Int4,
        stepnr -> Int4,
        name -> Text,
        path -> Text,
    }
}

table! {
    buildsteps (build, stepnr) {
        build -> Int4,
        stepnr -> Int4,
        #[sql_name = "type"]
        type_ -> Int4,
        drvpath -> Nullable<Text>,
        busy -> Int4,
        status -> Nullable<Int4>,
        errormsg -> Nullable<Text>,
        starttime -> Nullable<Int4>,
        stoptime -> Nullable<Int4>,
        machine -> Text,
        system -> Nullable<Text>,
        propagatedfrom -> Nullable<Int4>,
        overhead -> Nullable<Int4>,
        timesbuilt -> Nullable<Int4>,
        isnondeterministic -> Nullable<Bool>,
    }
}

table! {
    cachedbazaarinputs (uri, revision) {
        uri -> Text,
        revision -> Int4,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    cachedcvsinputs (uri, module, sha256hash) {
        uri -> Text,
        module -> Text,
        timestamp -> Int4,
        lastseen -> Int4,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    cacheddarcsinputs (uri, revision) {
        uri -> Text,
        revision -> Text,
        sha256hash -> Text,
        storepath -> Text,
        revcount -> Int4,
    }
}

table! {
    cachedgitinputs (uri, branch, revision) {
        uri -> Text,
        branch -> Text,
        revision -> Text,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    cachedhginputs (uri, branch, revision) {
        uri -> Text,
        branch -> Text,
        revision -> Text,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    cachedpathinputs (srcpath, sha256hash) {
        srcpath -> Text,
        timestamp -> Int4,
        lastseen -> Int4,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    cachedsubversioninputs (uri, revision) {
        uri -> Text,
        revision -> Int4,
        sha256hash -> Text,
        storepath -> Text,
    }
}

table! {
    failedpaths (path) {
        path -> Text,
    }
}

table! {
    jobs (project, jobset, name) {
        project -> Text,
        jobset -> Text,
        name -> Text,
    }
}

table! {
    jobsetevalinputs (eval, name, altnr) {
        eval -> Int4,
        name -> Text,
        altnr -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        uri -> Nullable<Text>,
        revision -> Nullable<Text>,
        value -> Nullable<Text>,
        dependency -> Nullable<Int4>,
        path -> Nullable<Text>,
        sha256hash -> Nullable<Text>,
    }
}

table! {
    jobsetevalmembers (eval, build) {
        eval -> Int4,
        build -> Int4,
        isnew -> Int4,
    }
}

table! {
    jobsetevals (id) {
        id -> Int4,
        project -> Text,
        jobset -> Text,
        timestamp -> Int4,
        checkouttime -> Int4,
        evaltime -> Int4,
        hasnewbuilds -> Int4,
        hash -> Text,
        nrbuilds -> Nullable<Int4>,
        nrsucceeded -> Nullable<Int4>,
    }
}

table! {
    jobsetinputalts (project, jobset, input, altnr) {
        project -> Text,
        jobset -> Text,
        input -> Text,
        altnr -> Int4,
        value -> Nullable<Text>,
        revision -> Nullable<Text>,
    }
}

table! {
    jobsetinputs (project, jobset, name) {
        project -> Text,
        jobset -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        emailresponsible -> Int4,
    }
}

table! {
    jobsetrenames (project, from_) {
        project -> Text,
        from_ -> Text,
        to_ -> Text,
    }
}

table! {
    jobsets (project, name) {
        name -> Text,
        project -> Text,
        description -> Nullable<Text>,
        nixexprinput -> Text,
        nixexprpath -> Text,
        errormsg -> Nullable<Text>,
        errortime -> Nullable<Int4>,
        lastcheckedtime -> Nullable<Int4>,
        triggertime -> Nullable<Int4>,
        enabled -> Int4,
        enableemail -> Int4,
        hidden -> Int4,
        emailoverride -> Text,
        keepnr -> Int4,
        checkinterval -> Int4,
        schedulingshares -> Int4,
        fetcherrormsg -> Nullable<Text>,
        forceeval -> Nullable<Bool>,
        starttime -> Nullable<Int4>,
    }
}

table! {
    newsitems (id) {
        id -> Int4,
        contents -> Text,
        createtime -> Int4,
        author -> Text,
    }
}

table! {
    nrbuilds (what) {
        what -> Text,
        count -> Int4,
    }
}

table! {
    projectmembers (project, username) {
        project -> Text,
        username -> Text,
    }
}

table! {
    projects (name) {
        name -> Text,
        displayname -> Text,
        description -> Nullable<Text>,
        enabled -> Int4,
        hidden -> Int4,
        owner -> Text,
        homepage -> Nullable<Text>,
        declfile -> Nullable<Text>,
        decltype -> Nullable<Text>,
        declvalue -> Nullable<Text>,
    }
}

table! {
    releasemembers (project, release_, build) {
        project -> Text,
        release_ -> Text,
        build -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    releases (project, name) {
        project -> Text,
        name -> Text,
        timestamp -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    starredjobs (username, project, jobset, job) {
        username -> Text,
        project -> Text,
        jobset -> Text,
        job -> Text,
    }
}

table! {
    systemstatus (what) {
        what -> Text,
        status -> Json,
    }
}

table! {
    systemtypes (system) {
        system -> Text,
        maxconcurrent -> Int4,
    }
}

table! {
    urirevmapper (baseuri) {
        baseuri -> Text,
        uri -> Text,
    }
}

table! {
    userroles (username, role) {
        username -> Text,
        role -> Text,
    }
}

table! {
    users (username) {
        username -> Text,
        fullname -> Nullable<Text>,
        emailaddress -> Text,
        password -> Text,
        emailonerror -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        publicdashboard -> Bool,
    }
}

joinable!(buildmetrics -> builds (build));
joinable!(buildmetrics -> projects (project));
joinable!(buildoutputs -> builds (build));
joinable!(buildproducts -> builds (build));
joinable!(builds -> projects (project));
joinable!(buildstepoutputs -> builds (build));
joinable!(jobs -> projects (project));
joinable!(jobsetevalinputs -> builds (dependency));
joinable!(jobsetevalinputs -> jobsetevals (eval));
joinable!(jobsetevalmembers -> builds (build));
joinable!(jobsetevalmembers -> jobsetevals (eval));
joinable!(jobsetevals -> projects (project));
joinable!(jobsetrenames -> projects (project));
joinable!(jobsets -> projects (project));
joinable!(newsitems -> users (author));
joinable!(projectmembers -> projects (project));
joinable!(projectmembers -> users (username));
joinable!(projects -> users (owner));
joinable!(releasemembers -> builds (build));
joinable!(releasemembers -> projects (project));
joinable!(releases -> projects (project));
joinable!(starredjobs -> projects (project));
joinable!(starredjobs -> users (username));
joinable!(userroles -> users (username));

allow_tables_to_appear_in_same_query!(
    aggregateconstituents,
    buildinputs,
    buildmetrics,
    buildoutputs,
    buildproducts,
    builds,
    buildstepoutputs,
    buildsteps,
    cachedbazaarinputs,
    cachedcvsinputs,
    cacheddarcsinputs,
    cachedgitinputs,
    cachedhginputs,
    cachedpathinputs,
    cachedsubversioninputs,
    failedpaths,
    jobs,
    jobsetevalinputs,
    jobsetevalmembers,
    jobsetevals,
    jobsetinputalts,
    jobsetinputs,
    jobsetrenames,
    jobsets,
    newsitems,
    nrbuilds,
    projectmembers,
    projects,
    releasemembers,
    releases,
    starredjobs,
    systemstatus,
    systemtypes,
    urirevmapper,
    userroles,
    users,
);
