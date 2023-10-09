
import { AgentPubKey, HoloHash,
	 ActionHash, EntryHash }	from '@spartan-hc/holo-hash';
import { Entity }			from '@spartan-hc/caps-entities';
import {
    intoStruct,
    OptionType, VecType, MapType,
}					from '@whi/into-struct';


//
// AppEntry Handling
//
export const AppStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function AppEntry ( entry ) {
    return intoStruct( entry, AppStruct );
}


//
// WebAppEntry Handling
//
export const WebAppStruct = {
    "manifest":			Object,
    "resources":		Object,
};

export function WebAppEntry ( entry ) {
    return intoStruct( entry, WebAppStruct );
}


//
// WebAppPackageEntry Handling
//
export const WebAppPackageStruct = {
    "title":			String,
    "subtitle":			String,
    "description":		String,
    "icon":			EntryHash,
    "source_code_url":		String,
    "maintainer":		{
	"group":		OptionType([ ActionHash, ActionHash ]),
	"agent":		OptionType( AgentPubKey ),
    },
    "deprecation":		OptionType( Object ),
    "metadata":			Object,
};

export function WebAppPackageEntry ( entry ) {
    return intoStruct( entry, WebAppPackageStruct );
}

export class WebAppPackage extends Entity {
    constructor ( entity ) {
	entity.content		= intoStruct( entity.content, WebAppPackageStruct );
	super( entity );
    }
}

export default {
    AppStruct,
    AppEntry,

    WebAppStruct,
    WebAppEntry,

    WebAppPackageStruct,
    WebAppPackageEntry,
    WebAppPackage,
};
