var context = $evaluation.getContext();

var contextAttributes = context.getAttributes();

var visibilityAttr = contextAttributes.getValue('visibility');
var visibility = visibilityAttr ? visibilityAttr.asString(0) : null;

if ('Public' === visibility) {
    $evaluation.grant();
}