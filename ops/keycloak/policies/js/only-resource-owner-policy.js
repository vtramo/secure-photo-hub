var context = $evaluation.getContext();

var identity = context.getIdentity();
var identityAttributes = identity.getAttributes();
var sub = identityAttributes.getValue('sub').asString(0);

var contextAttributes = context.getAttributes();

var resourceOwnerAttr = contextAttributes.getValue('resourceOwner');
var resourceOwner = resourceOwnerAttr ? resourceOwnerAttr.asString(0) : null;

if (sub === resourceOwner) {
    $evaluation.grant();
}